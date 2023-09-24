provider "aws" {
  region = "us-east-1" # Change this to your desired AWS region
}

variable "vpc_id" {
  type = string
  default = ""
}

variable "subnet_id" {
  type = string
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


resource "aws_instance" "ibc-deployer" {
  ami           = data.aws_ami.ubuntu.id
  instance_type = "t2.micro" 
  key_name      = "root_ssh_pub_key"

  subnet_id             = var.subnet_id

  tags = {
    Name = "ContractDeployerExample"
    Project = "IBC"
  }

  user_data = data.template_file.init_script.rendered
}

output "public_ip" {
  value = aws_instance.ibc-deployer.public_ip
}
output "private_ip" {
  value = aws_instance.ibc-deployer.private_ip
}