#!/bin/bash
SSH_PUBKEY="SSH_PUBKEY_HERE"
CIPHER_TEXT="CIPHER_TEXT_HERE"
DEPLOYR_HOME="/home/deployr"

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

# Setup ENV and command history loging
echo '# Export ICON node environment
export GOROOT=/usr/local/go
export GOPATH=/opt/ibc
export JAVA_HOME=/opt/java/jdk-11.0.18+10
export PATH="$PATH:${GOROOT}/bin:${JAVA_HOME}/bin:${GOPATH}/bin"
# Shell timeout policy to 3 min
TMOUT=180
readonly TMOUT
export TMOUT

# Log shell commands
# Set PROMPT_COMMAND to log every command to syslog
PROMPT_COMMAND='history -a >(logger -t "[$USER] $SSH_CONNECTION")'' >> /etc/profile

# Configure auditd
echo '-a always,exit -F arch=b64 -S execve -k command-exec
-a always,exit -F arch=b32 -S execve -k command-exec' > /etc/audit/rules.d/audit_commands.rules

apt-get install auditd audispd-plugins
systemctl enable auditd
systemctl start auditd

# Create Directories
mkdir -p /opt/deployer
mkdir -p /opt/deployer{bin,root}
mkdir -p /opt/deployer/root/{keystore,keyutils}

# Clone repo
cd /opt/deployer/root/ && git clone https://github.com/izyak/icon-ibc.git
cp -r icon-ibc/deployer/* /opt/deployer

# Create user & configure ssh access
useradd -m -d ${DEPLOYR_HOME} -s /bin/bash deployr
mkdir ${DEPLOYR_HOME}/.ssh
echo "$SSH_PUBKEY" | base64 -d > ${DEPLOYR_HOME}/.ssh/authorized_keys
echo "$CIPHER_TEXT" | base64 -d > /opt/deployer/root/.cipher_text
# Create Aliases for the user 'deployr'
echo '## Aliases
alias fetch-walletkeys='sudo /opt/deployer/bin/fetch_keys.sh'
alias pull-deploy-script='sudo /opt/deployer/bin/update_git.sh'
alias check-env='sudo /opt/deployer/bin/check-paramener.sh'
alias make='sudo /opt/deployer/bin/deploy.sh'' >> ${DEPLOYR_HOME}/.bashrc



