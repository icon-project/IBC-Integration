#!/bin/bash
exec 3>&1 4>&2
trap 'exec 2>&4 1>&3' 0 1 2 3
exec 1>/tmp/user_data_log.out 2>&1

# Below variable is resolved during the rendering of the Terraform template file.
SSH_PUBKEY="SSH_PUBKEY_HERE"
CIPHER_TEXT="CIPHER_TEXT_HERE"
DEPLOY_SCRIPT_BRANCH="DEPLOY_SCRIPT_BRANCH_HERE"  # Deployment repo: https://github.com/izyak/icon-ibc.git
KMS_ID="KMS_ID_HERE"
DEPLOYR_HOME="/home/deployr"
GO_VERS="1.20.6"
JAVA_VERS="11.0.18_10"
ARCHWAY_VERS="0.4.0"

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

apt-get install auditd audispd-plugins -y
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

# Clone repo
cd /opt/deployer/root/ && git clone https://github.com/izyak/icon-ibc.git
cd icon-ibc
git checkout $${DEPLOY_SCRIPT_BRANCH}
cd ..
cp -r icon-ibc/deployer/* /opt/deployer

# Create user & configure ssh access
useradd -m -d $${DEPLOYR_HOME} -s /bin/bash deployr
mkdir $${DEPLOYR_HOME}/.ssh
echo "$SSH_PUBKEY" > $${DEPLOYR_HOME}/.ssh/authorized_keys

## Don't show Cipher text in the log
set +x
echo -n "$CIPHER_TEXT" | base64 -d > /opt/deployer/root/keyutils/.cipher_text
echo -n "$KMS_ID" > /opt/deployer/root/keyutils/kms_id
chmod -R 400 /opt/deployer/root/keystore
set -x

cd /tmp
# Install go
wget -q https://go.dev/dl/go$${GO_VERS}.linux-amd64.tar.gz
tar xf go$${GO_VERS}.linux-amd64.tar.gz -C /usr/local

# Install Java
wget -q https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.18%2B10/OpenJDK11U-jdk_x64_linux_hotspot_$${JAVA_VERS}.tar.gz
tar xf OpenJDK11U-jdk_x64_linux_hotspot_$${JAVA_VERS}.tar.gz -C /opt/java

# Install goloop
go install github.com/icon-project/goloop/cmd/goloop@latest

# Install archway
wget -q https://github.com/archway-network/archway/releases/download/v$${ARCHWAY_VERS}/archway_$${ARCHWAY_VERS}_linux_amd64.tar.gz
tar xf archway_$${ARCHWAY_VERS}_linux_amd64.tar.gz -C /usr/local/bin

# Install boto3, yq, and jq
apt-get install python3-pip -y
pip3 install boto3
apt-get install jq -y
wget -qO /usr/local/bin/yq https://github.com/mikefarah/yq/releases/latest/download/yq_linux_amd64
chmod +x /usr/local/bin/yq

cd - 

# Configure sudo
echo 'deployr ALL=(ALL) NOPASSWD: /opt/deployer/bin/run.sh
deployr ALL=(ALL) NOPASSWD: /opt/deployer/bin/fetch_keys.sh
deployr ALL=(ALL) NOPASSWD: /opt/deployer/bin/update_git.sh
deployr ALL=(ALL) NOPASSWD: /opt/deployer/bin/deploy.sh
deployr ALL=(ALL) NOPASSWD: /opt/deployer/bin/check-paramener.sh' > /etc/sudoers.d/deployr_sudo_commands

# Add goloop binary path to secure path
sed -i '/secure_path/ s/"$/:\/usr\/local\/go\/bin:\/opt\/ibc\/bin:\/opt\/java\/jdk-11.0.18+10\/bin"/' /etc/sudoers

# Create Aliases for the user 'deployr'
echo "## Aliases
alias fetch-walletkeys='sudo /opt/deployer/bin/fetch_keys.sh'
alias pull-deploy-script='sudo /opt/deployer/bin/update_git.sh'
alias check-env='sudo /opt/deployer/bin/check-parameter.sh'
alias make='sudo /opt/deployer/bin/deploy.sh'" >> $${DEPLOYR_HOME}/.bashrc



