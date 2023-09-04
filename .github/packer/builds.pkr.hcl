build {
  name    = "test"
  sources = ["source.amazon-ebs.linux"]

  provisioner "shell" {
    execute_command = "sudo -S bash -c '{{ .Path }}'"
    script = "${path.root}/builder.sh"
  }
}