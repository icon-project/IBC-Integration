#!/bin/bash
exec 3>&1 4>&2
trap 'exec 2>&4 1>&3' 0 1 2 3
exec 1>/tmp/user_data_log.out 2>&1

# Below variable is resolved during the rendering of the Terraform template file.
SSH_PUBKEY="SSH_PUBKEY_HERE"
CIPHER_TEXT="CIPHER_TEXT_HERE"
export GITHUB_ACCESS_TOKEN="GITHUB_TOKEN_HERE"
export CI_USER="CI_USER_HERE"
DEPLOY_SCRIPT_BRANCH="DEPLOY_SCRIPT_BRANCH_HERE"  # Deployment repo: https://github.com/izyak/icon-ibc.git
KMS_ID="KMS_ID_HERE"
DEPLOYR_HOME="/home/deployr"
GO_VERS="1.20.6"
JAVA_VERS="11.0.18_10"
ARCHWAY_VERS="7.0.0"
INJECTIVE_VERS="1.12.1-1705909076"
NEUTRON_VERS="3.0.2"
SUI_VERS="mainnet-v1.30.1"

set -x
export GOROOT=/usr/local/go
export GOPATH=/opt/ibc
export /root/.cache/go-build
export JAVA_HOME=/opt/java/jdk-11.0.18+10
export GOCACHE=/root/go/cache
export PATH="$PATH:$${GOROOT}/bin:$${JAVA_HOME}/bin:$${GOPATH}/bin"


echo "Installing prerequities binaries..."

# Disable ipv6
echo "# Disable ipv6
net.ipv6.conf.all.disable_ipv6=1
net.ipv6.conf.default.disable_ipv6=1
net.ipv6.conf.lo.disable_ipv6=1" >> /etc/sysctl.conf

sysctl -p

#Update OS Packages
apt-get update
apt-get upgrade -y
apt-get install expect -y

cat << 'EOF' >> /etc/profile
# Setup ENV and command history loging
# Export ICON node environment
export GOROOT=/usr/local/go
export GOPATH=/opt/ibc
export JAVA_HOME=/opt/java/jdk-11.0.18+10
export PATH="$PATH:$${GOROOT}/bin:$${JAVA_HOME}/bin:$${GOPATH}/bin"
# Shell timeout policy to 3 min
TMOUT=180
readonly TMOUT
export TMOUT

# Log shell commands
# Set PROMPT_COMMAND to log every command to syslog
PROMPT_COMMAND='history -a >(logger -t "[$USER] $SSH_CONNECTION")'
EOF

apt-get install auditd audispd-plugins unzip -y
systemctl enable auditd
systemctl start auditd



# Configure auditd
echo '-a always,exit -F arch=b64 -S execve -k command-exec
-a always,exit -F arch=b32 -S execve -k command-exec' >> /etc/audit/audit.rules
systemctl restart auditd

# Create Directories
mkdir /opt/java
mkdir -p /opt/deployer
mkdir -p /opt/deployer/{bin,root}
mkdir -p /opt/deployer/root/{keystore,keyutils}

mkdir -p /root/.ssh
echo "$${GITHUB_ACCESS_TOKEN}" | base64 -d > /root/.ssh/id_rsa
chmod 600 /root/.ssh/id_rsa
ssh-keyscan github.com >> /root/.ssh/known_hosts
git clone git@github.com:icon-project/devnet.git /opt/deployer/root/ibc-devops

cd /opt/deployer/root/ibc-devops
git checkout $${DEPLOY_SCRIPT_BRANCH}
cd ..
mv /ibc-devops /opt/deployer/root
cd /opt/deployer/root/
cp -r ibc-devops/Deployments/relayer/deployer/* /opt/deployer


# Create user & configure ssh access
useradd -m -d $${DEPLOYR_HOME} -s /bin/bash deployr
mkdir $${DEPLOYR_HOME}/.ssh
echo "$SSH_PUBKEY" > $${DEPLOYR_HOME}/.ssh/authorized_keys

## Don't show Cipher text in the log
set +x
echo -n "$CIPHER_TEXT" | base64 -d > /opt/deployer/root/keyutils/.cipher_text
echo -n "$KMS_ID" > /opt/deployer/root/keyutils/kms_id
chmod -R 400 /opt/deployer/root/keyutils/.cipher_text
chmod 770 /opt/deployer/root/keystore
set -x

cd /tmp
# Install go
sysctl -p
wget -q https://go.dev/dl/go$${GO_VERS}.linux-amd64.tar.gz
tar xf go$${GO_VERS}.linux-amd64.tar.gz -C /usr/local

# Install Java
wget -q https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.18%2B10/OpenJDK11U-jdk_x64_linux_hotspot_$${JAVA_VERS}.tar.gz
tar xf OpenJDK11U-jdk_x64_linux_hotspot_$${JAVA_VERS}.tar.gz -C /opt/java

# Install goloop
go install github.com/icon-project/goloop/cmd/goloop@latest

# Install archway
wget -q https://github.com/archway-network/archway/releases/download/v$${ARCHWAY_VERS}/archwayd_v$${ARCHWAY_VERS}_linux_amd64.zip
unzip archwayd_v$${ARCHWAY_VERS}_linux_amd64.zip
sudo cp archwayd /usr/local/bin

# Install injectived
wget -q https://github.com/InjectiveLabs/injective-chain-releases/releases/download/v$${INJECTIVE_VERS}/linux-amd64.zip
unzip linux-amd64.zip
sudo cp injectived peggo /usr/bin
sudo cp libwasmvm.x86_64.so /usr/lib
sudo chmod +x /usr/bin/injectived
sudo chmod +x /usr/bin/peggo

# Install neutron
wget -q https://github.com/neutron-org/neutron/releases/download/v$${NEUTRON_VERS}/neutrond-linux-amd64
sudo cp neutrond-linux-amd64 /usr/local/bin/neutrond
sudo chmod +x /usr/local/bin/neutrond

# Install sui
wget -q https://github.com/MystenLabs/sui/releases/download/$${SUI_VERS}/sui-$${SUI_VERS}-ubuntu-x86_64.tgz
sudo tar xf sui-$${SUI_VERS}-ubuntu-x86_64.tgz -C /usr/local/bin
sudo chmod +x /usr/local/bin/sui

# Install Dasel
sudo wget -qO /usr/local/bin/dasel https://github.com/TomWright/dasel/releases/latest/download/dasel_linux_amd64
sudo chmod a+x /usr/local/bin/dasel

# Install boto3, yq, and jq
apt-get install python3-pip -y
pip3 install boto3 pwinput solana
apt-get install jq -y
wget -qO /usr/local/bin/yq https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64
chmod +x /usr/local/bin/yq

## Install rust
echo "Installing cargo"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rustup-init.sh

cat << 'EOF' > cargo.expect
#!/usr/bin/expect
set timeout -1
spawn sh rustup-init.sh
expect "1) Proceed with standard installation (default - just press enter)"
send "\r"
expect {
    "2) Customize installation" {
        send "\r"
        exp_continue
    }
    "3) Cancel installation" {
        send "\r"
        exp_continue
    }
    eof
}
EOF

chmod +x cargo.expect
./cargo.expect

cd - 

# Create sui client config
mkdir -p /root/.sui/sui_config
cat <<EOF > /root/.sui/sui_config/sui.keystore
keystore:
  File: /root/.sui/sui_config/sui.keystore
envs:
  - alias: testnet
    rpc: "https://fullnode.testnet.sui.io:443"
    ws: ~
    basic_auth: ~
  - alias: mainnet
    rpc: "https://fullnode.mainnet.sui.io:443"
    ws: ~
    basic_auth: ~
active_env: mainnet
active_address: "0x539c665cd9899d040c56756df8f7ed34649ab6aeae28da5cb07d3274dc9f9d36"
EOF

# Configure sudo
echo 'deployr ALL=(ALL) NOPASSWD: /opt/deployer/bin/run.sh
deployr ALL=(ALL) NOPASSWD: /opt/deployer/bin/fetch_keys.sh
deployr ALL=(ALL) NOPASSWD: /opt/deployer/bin/update_git.sh
deployr ALL=(ALL) NOPASSWD: /opt/deployer/bin/deploy.sh
deployr ALL=(ALL) NOPASSWD: /opt/deployer/bin/check-parameter.sh' > /etc/sudoers.d/deployr_sudo_commands

# Add chain command binary path to secure path
sed -i '/secure_path/ s/"$/:\/usr\/local\/go\/bin:\/opt\/ibc\/bin:\/opt\/java\/jdk-11.0.18+10\/bin:\/root\/.local\/share\/solana\/install\/active_release\/bin:\/root\/.cargo\/bin\/"/' /etc/sudoers
# Create Aliases for the user 'deployr'
echo "## Aliases
alias fetch-walletkeys='sudo /opt/deployer/bin/fetch_keys.sh'
alias pull-deploy-script='sudo /opt/deployer/bin/update_git.sh'
alias check-env='sudo /opt/deployer/bin/check-parameter.sh'
alias make='sudo /opt/deployer/bin/deploy.sh'" >> $${DEPLOYR_HOME}/.bashrc

chmod +x /opt/deployer/root/keyutils/add_secret.sh
echo "## Aliases
alias add-secrets='/opt/deployer/root/keyutils/add_secret.sh'
alias add-stellar-key='/opt/deployer/root/keyutils/add_stellar_key.sh'" >> /root/.bashrc

## Install solana
source "/root/.cargo/env"

export PATH=$${PATH}:/root/.cargo/bin
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
/root/.cargo/bin/cargo install --git https://github.com/coral-xyz/anchor --tag v0.30.1 anchor-cli
sudo apt-get install npm -y
npm install -g yarn
echo "export PATH=$${PATH}:/root/.local/share/solana/install/active_release/bin" >> /root/.bashrc

## Install multisig
cargo install --git https://github.com/icon-project/cw-plus.git --branch feat/test-multisig cwmultisig

## Install Stellar
wget https://github.com/stellar/stellar-cli/releases/download/v21.4.1/stellar-cli-21.4.1-x86_64-unknown-linux-gnu.tar.gz
tar -xvzf stellar-cli-21.4.1-x86_64-unknown-linux-gnu.tar.gz 
mv stellar /usr/local/bin/stellar

chmod 400 /tmp/user_data_log.out || true
