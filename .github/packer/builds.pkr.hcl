build {
  name    = local.name
  sources = ["source.amazon-ebs.linux"]

  provisioner "file" {
    source      = "${path.root}/relayer.Dockerfile"
    destination = "/tmp/Dockerfile"
  }

  provisioner "shell" {
    execute_command = "sudo -S bash -c '{{ .Path }}'"
    script = "${path.root}/builder.sh"
  }
}