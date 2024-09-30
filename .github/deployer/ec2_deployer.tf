provider "aws" {
  region = "us-east-1" # Change this to your desired AWS region
}

terraform {
  backend "s3" {
    bucket         = "logrotate-ibc-contract-deployer"
    key            = "terraform.tfstate"
    region         = "us-east-1"
    encrypt        = true
    workspace_key_prefix = "ibc-deployer-environment"
  }
}

variable "vpc_id" {
  type = string
  default = ""
}

variable "subnet_id" {
  type = string
  default = ""
}

variable "vpc_security_group_ids" {
  type    = string
  default = ""
}

variable "root_ssh_pub_key" {
  type = string
  default = ""
}
variable "deployr_ssh_pub_key" {
  type = string
  default = ""
}

data "template_file" "init_script" {
  template = file("init_script.sh") 
}

resource "aws_key_pair" "deployer_root_key_testnet" {
  key_name = "deployer_root_key_testnet"
  public_key = file("./id_rsa.pub")
  
}

data "aws_ami" "ubuntu" {
    most_recent = true
filter {
        name   = "name"
        values = ["ubuntu/images/hvm-ssd/*20.04-amd64-server-*"]
    }
filter {
        name   = "virtualization-type"
        values = ["hvm"]
    }
owners = ["099720109477"]
}


locals {
  parsed_security_groups = split(" ", var.vpc_security_group_ids)
}

resource "aws_instance" "ibc-deployer-testnet" {
  ami           = data.aws_ami.ubuntu.id
  instance_type = "t2.medium" 
  key_name      = "deployer_root_key_testnet"

  subnet_id             = var.subnet_id
  vpc_security_group_ids = local.parsed_security_groups
  iam_instance_profile = "SecretManagerReadAccess-ibc"
  # associate_public_ip_address = false
  root_block_device {
    volume_size = 25
  }
  tags = {
    Name = "testnet-deployer-machine"
    Environment = "lisbon"
    Project = "IBC"
  }

  user_data = data.template_file.init_script.rendered
}

output "public_ip" {
  value = aws_instance.ibc-deployer-testnet.public_ip
}
output "private_ip" {
  value = aws_instance.ibc-deployer-testnet.private_ip
}
