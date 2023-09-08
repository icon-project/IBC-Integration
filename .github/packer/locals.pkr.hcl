locals {
  timestamp = regex_replace(timestamp(), "[- TZ:]", "")
  name = "github-runner"
}