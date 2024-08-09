#!/usr/bin/expect -f

set timeout -1
set username [lindex $argv 0]
set token [lindex $argv 1]
set repo_url "https://github.com/icon-project/devnet.git"
set target_dir "ibc-devops"

# Clone the repository
spawn git clone $repo_url $target_dir
expect "Username for 'https://github.com':"
send "$username\r"
expect "Password for 'https://$username@github.com':"
send "$token\r"
expect eof
