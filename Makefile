# Determine this makefile's path.
# Be sure to place this BEFORE `include` directives, if any.
# THIS_FILE := $(lastword $(MAKEFILE_LIST))
THIS_FILE := $(abspath $(lastword $(MAKEFILE_LIST)))
CURRENT_DIR := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

include vars.mk

.PHONY: docker-build
docker-build: ## build and publish CPE Docker image locally
	docker buildx build \
		--progress auto \
		-t $(DOCKER_IMAGE) -t $(DOCKER_REPO):latest \
		--load \
		.

.PHONY: docker-publish
docker-publish: ## publish Docker image to Docker-Hub
	docker buildx build \
		--progress auto \
		--platform linux/amd64,linux/arm64 \
		-t $(DOCKER_IMAGE) -t $(DOCKER_REPO):latest \
		--push \
		.

.PHONY: docker-run
docker-run: ## compile and run app locally
	docker run --rm -it -p 4200:4200 $(DOCKER_IMAGE)

.PHONY: docker-image-tag
docker-image-tag: ## prints the docker image tag
	@echo $(BUILD_TAG)

.PHONY: check
check: ## run all checks
	@echo "Running all checks"
	@echo "Running 'cargo +nightly fmt --check'"
	@cargo +nightly fmt --check
	@echo "Running 'cargo clippy -- -D warnings'"
	@cargo clippy -- -D warnings

.PHONY: build
build: check ## build app
	@echo "Running 'cargo build'"
	@cargo build

.PHONY: help
help: ## this help
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST) | sort

.DEFAULT_GOAL := help
