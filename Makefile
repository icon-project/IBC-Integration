#!/usr/bin/make -f
DOCKER := $(shell which docker)
DOCKER_BUF := $(DOCKER) run --rm -v $(CURDIR):/workspace --workdir /workspace bufbuild/buf:1.9.0
PROJECT_NAME = $(shell git remote get-url origin | xargs basename -s .git)

export GO111MODULE = on


###############################################################################
###                                Protobuf                                 ###
###############################################################################

protoVer=0.11.1
protoImageName=ghcr.io/cosmos/proto-builder:$(protoVer)
builderImage=contract-builder
containerBuilder=$(PROJECT_NAME)-optimize-builder-img
containerProtoGenGo=$(PROJECT_NAME)-proto-gen-go-$(protoVer)
containerProtoGenRust=$(PROJECT_NAME)-proto-gen-rust-$(protoVer)
containerProtoFmt=$(PROJECT_NAME)-proto-fmt-$(protoVer)

proto-all: proto-format proto-lint proto-gen

proto-format:
	@echo "Formatting Protobuf files"
	@if docker ps -a --format '{{.Names}}' | grep -Eq "^${containerProtoFmt}$$"; then docker start -a $(containerProtoFmt); else docker run --name $(containerProtoFmt) -v $(CURDIR):/workspace --workdir /workspace $(protoImageName) \
		find ./ -name "*.proto" -exec clang-format -i {} \; ; fi

proto-lint:
	@$(DOCKER_BUF) lint --error-format=json

proto-gen-go:
	@echo "Generating Go Protobuf files"
	@if docker ps -a --format '{{.Names}}' | grep -Eq "^${containerProtoGenGo}$$"; then docker start -a $(containerProtoGenGo); else docker run   --name $(containerProtoGenGo) -v $(CURDIR):/workspace --workdir /workspace $(protoImageName) \
		sh ./scripts/protocgen_go.sh; fi

proto-gen-rust:
	@echo "Generating Rust Protobuf files"
	@if docker ps -a --format '{{.Names}}' | grep -Eq "^${containerProtoGenRust}$$"; then docker start -a $(containerProtoGenRust); else docker run  --name $(containerProtoGenRust) -v $(CURDIR):/workspace --workdir /workspace $(protoImageName) \
		sh ./scripts/protocgen_rust.sh; fi

build-builder-img:
	@echo "Build builder image"
	docker build -t "${builderImage}" . -f ./scripts/.DockerfileContractBuilder

optimize-jar:
	@echo "Generating optimized jar for ICON contracts"
	@if docker ps -a --format '{{.Names}}' | grep -Eq "^${containerBuilder}-icon$$"; then docker start -a "${containerBuilder}-icon" ; else docker run  --name "${containerBuilder}-icon"   -v $(CURDIR):/workspace --workdir /workspace $(builderImage) sh ./scripts/optimize-jar.sh; fi

optimize-cosmwasm:
	@echo "Generating optimized cosmwasm for Archway contracts"
	@if docker ps -a --format '{{.Names}}' | grep -Eq "^${containerBuilder}-cosmwasm$$"; then docker start -a "${containerBuilder}-cosmwasm" ; else docker run  --name "${containerBuilder}-cosmwasm"  -v $(CURDIR):/workspace --workdir /workspace $(builderImage) sh ./scripts/optimize-cosmwasm.sh; fi

optimize-xcall:
	@echo "Generating optimized xcall cosmwasm for Archway contracts"
	@if docker ps -a --format '{{.Names}}' | grep -Eq "^${containerBuilder}-xcall$$"; then docker start -a "${containerBuilder}-xcall" ; else docker run  --name "${containerBuilder}-xcall" -v $(CURDIR):/workspace --workdir /workspace $(builderImage) sh ./scripts/build-xcall.sh; fi


optimize-build:
	@echo "Generating optimized contracts..."
	@if docker ps -a --format '{{.Names}}' | grep -Eq "^${containerBuilder}$$"; then docker start -a ${containerBuilder}; else docker run --name $(containerBuilder) -v $(CURDIR):/workspace --workdir /workspace $(builderImage) sh ./scripts/optimize-build.sh; fi

gobuild:
	go build .

.PHONY: proto-all proto-gen proto-gen-any proto-swagger-gen proto-format proto-lint proto-check-breaking proto-update-deps gobuild
