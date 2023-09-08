#!/bin/sh

set -euof pipefail

install_deps() {
    sudo yum install -y git docker make tar perl-Digest-SHA libicu
}

configure_docker() {
    sudo systemctl enable docker --now
    sudo usermod -aG docker ec2-user
}

build_goloop_images() {
  git clone https://github.com/icon-project/goloop.git
  make goloop-image -C goloop
  make goloop-icon-image -C goloop
  docker tag goloop iconloop/goloop-icon
}

pull_archway_images() {
  docker pull archwaynetwork/archwayd:sha-8f53ac8
}

configure_relayer() {
  cat <<EOT > ~/build.sh
  #!/bin/sh
  docker build -t relayer --build-arg VERSION=\$1 .
EOT
  chmod +x ~/build.sh
}

build_services() {
  build_goloop_images
  pull_archway_images
  configure_relayer
}

cleanup(){
  rm -rf goloop
  sudo yum remove -y tar perl-Digest-SHA
  sudo yum clean all
}

install_deps
configure_docker
build_services
cleanup