build {
  name    = local.name
  sources = ["source.amazon-ebs.linux"]

  provisioner "file" {
    source      = "${path.root}/relayer.Dockerfile"
    destination = "Dockerfile"
  }

  provisioner "shell" {
    execute_command = "sudo -S sh -c '{{ .Path }}'"
    script = "${path.root}/builder.sh"
  }
}