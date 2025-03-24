SHELL :=/bin/bash -e -o pipefail
PWD   := $(shell pwd)

.DEFAULT_GOAL := all
.PHONY: all
all: ## build pipeline
all: generate format check test

.PHONY: ci
ci: ## CI build pipeline
ci: all

.PHONY: precommit
precommit: ## validate the branch before commit
precommit: all

.PHONY: help
help:
	@echo 'Usage: make <OPTIONS> ... <TARGETS>'
	@echo ''
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: version
version: ## Check flutter version
	@rustc --version
	@cargo --version
	@cargo fmt --version
	@cargo clippy --version

.PHONY: get
get: ## Get the dependencies
	@

.PHONY: format
format: ## Format the code
	@

.PHONY: fmt
fmt: format

.PHONY: fix
fix: format ## Fix the code
	@

.PHONY: analyze
analyze: get ## Analyze the code
	@

.PHONY: check
check: get ## Check the code
	@

.PHONY: update
update: get ## Update dependencies
	@cargo update

.PHONY: upgrade-major
upgrade-major: get ## Upgrade to major versions
	@

.PHONY: outdated
outdated: get ## Check for outdated dependencies
	@

.PHONY: test
test: get ## Run the tests
	@

.PHONY: coverage
coverage: get ## Generate the coverage report
	@

.PHONY: generate
generate: get format ## Generate the code
	@

.PHONY: gen
gen: generate

.PHONY: codegen
codegen: generate

.PHONY: build
build: get format ## Build the project
	@echo "Building the project..."
	@cargo build --release
	@echo "Build artifacts are located in the target/release directory."

.PHONY: clean
clean: ## Clean the project and remove all generated files
	@rm -f coverage.*
	@rm -rf dist bin out build target

.PHONY: diff
diff: ## git diff
	$(call print-target)
	@git diff --exit-code
	@RES=$$(git status --porcelain) ; if [ -n "$$RES" ]; then echo $$RES && exit 1 ; fi
