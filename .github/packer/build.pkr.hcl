build {
  name    = local.name
  sources = ["source.amazon-ebs.amazon-linux"]

  provisioner "shell" {
    execute_command = "sudo -S bash -c '{{ .Path }}'"
    script = "${path.root}/builder.sh"
  }
}