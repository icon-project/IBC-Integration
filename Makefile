#!/usr/bin/make -f
DOCKER := $(shell which docker)
DOCKER_BUF := $(DOCKER) run --rm -v $(CURDIR):/workspace --workdir /workspace bufbuild/buf:1.9.0
PROJECT_NAME = $(shell git remote get-url origin | xargs basename -s .git)
BRANCH ?= "main"
export GO111MODULE = on

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
	@if docker ps -a --format '{{.Names}}' | grep -Eq "^${containerProtoFmt}$$"; then \
		docker start -a $(containerProtoFmt); \
	else \
		docker run --name $(containerProtoFmt) -v $(CURDIR):/workspace --workdir /workspace $(protoImageName) \
			find ./ -name "*.proto" -exec clang-format -i {} \;; \
	fi

proto-lint:
	@$(DOCKER_BUF) lint --error-format=json

proto-gen-go:
	@echo "Generating Go Protobuf files"
	@if docker ps -a --format '{{.Names}}' | grep -Eq "^${containerProtoGenGo}$$"; then \
		docker start -a $(containerProtoGenGo); \
	else \
		docker run --name $(containerProtoGenGo) -v $(CURDIR):/workspace --workdir /workspace $(protoImageName) \
			sh ./scripts/protocgen_go.sh; \
	fi

proto-gen-rust:
	@echo "Generating Rust Protobuf files"
	@if docker ps -a --format '{{.Names}}' | grep -Eq "^${containerProtoGenRust}$$"; then \
		docker start -a $(containerProtoGenRust); \
	else \
		docker run --name $(containerProtoGenRust) -v $(CURDIR):/workspace --workdir /workspace $(protoImageName) \
			sh ./scripts/protocgen_rust.sh; \
	fi

build-builder-img:
	@echo "Build builder image"
	docker build -t "${builderImage}" . -f ./scripts/.DockerfileContractBuilder

optimize-jar:
	@echo "Generating optimized jar for ICON contracts"
	@if docker ps -a --format '{{.Names}}' | grep -Eq "^${containerBuilder}-icon$$"; then \
		docker container rm "${containerBuilder}-icon"; \
	fi
	docker run --name "${containerBuilder}-icon" -v $(CURDIR):/workspace --workdir /workspace $(builderImage) bash ./scripts/optimize-jar.sh;

optimize-cosmwasm:
	@echo "Generating optimized cosmwasm for Archway contracts"
	@if docker ps -a --format '{{.Names}}' | grep -Eq "^${containerBuilder}-cosmwasm$$"; then \
		docker container rm "${containerBuilder}-cosmwasm"; \
	fi
	docker run --name "${containerBuilder}-cosmwasm" -v $(CURDIR):/workspace --workdir /workspace $(builderImage) bash ./scripts/optimize-cosmwasm.sh

optimize-xcall:
	@echo "Generating optimized xcall contracts ..."
	@if docker ps -a --format '{{.Names}}' | grep -Eq "^${containerBuilder}-xcall$$"; then \
		docker container rm "${containerBuilder}-xcall"; \
	fi
	docker run --name "${containerBuilder}-xcall" -v $(CURDIR):/workspace --workdir /workspace $(builderImage) bash ./scripts/optimize-xcall-build.sh build $(BRANCH);

optimize-build:
	@echo "Generating optimized contracts..."
	@if docker ps -a --format '{{.Names}}' | grep -Eq "^${containerBuilder}$$"; then \
		docker container rm ${containerBuilder}; \
	fi
	docker run --name $(containerBuilder) -v $(CURDIR):/workspace --workdir /workspace $(builderImage) bash ./scripts/optimize-build.sh build;

gobuild:
	go build .

e2e:
	@echo "Running e2e tests..."
	go test -v ./test/e2e -testify.m TestE2E_all

e2e-hopchain:
	@echo "Running hopchain e2e tests..."	
	go test -v ./test/e2e-hopchain -testify.m TestE2E_hopchain

e2e-demo-setup:
	@echo "Configuring e2e demo..."
	export PRESERVE_DOCKER=true && \
	go test -v ./test/e2e-demo -testify.m TestSetup

e2e-demo-clean:
	go test -v ./test/e2e-demo -testify.m TestCleanup

.PHONY: proto-all proto-gen proto-gen-any proto-swagger-gen proto-format proto-lint proto-check-breaking proto-update-deps gobuild optimize-build optimize-xcall e2e-demo-setup e2e-demo-clean
