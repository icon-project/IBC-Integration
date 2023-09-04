# source ami to use

source "amazon-ebs" "linux" {
  ami_name      = local.name
  instance_type = var.instance_type
  region        = var.aws_region
  ssh_username = var.ssh_username
  source_ami_filter {
    filters = {
      name                = "al2023-ami-2023.*.*-kernel-6.*-x86_64"
      root-device-type    = "ebs"
      virtualization-type = "hvm"
    }
    most_recent = true
    owners      = ["amazon"]
  }
  tags = {
    Name = local.name
    Project = "icon"
    ManagedBy = "Packer"
  }
}