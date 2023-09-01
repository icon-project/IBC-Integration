variable "aws_region" {
  type    = string
  default = "us-east-1"
}

variable "instance_type" {
  type    = string
  default = "t3.medium"
}

variable "ssh_username" {
  type    = string
  default = "ec2-user"
}

variable "github_runner_version" {
  type    = string
  default = "2.308.0"
}