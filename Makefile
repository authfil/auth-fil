# Authloom task runner. Run `make` or `make help` to list targets.
#
# The same targets run on the host, inside the dev container and in CI, so
# "works on my machine" and "passes CI" mean the same thing.

SHELL := /usr/bin/env bash
.SHELLFLAGS := -euo pipefail -c
.DEFAULT_GOAL := help

CARGO     ?= cargo
NPM       ?= npm
PYTHON    ?= python3
COMPOSE   ?= docker compose

NODE_ADAPTER   := adapters/node
PY_ADAPTER     := adapters/python
CONFORMANCE    := conformance
DOCS_SITE      := docs
FUZZ_DIR       := fuzz
FUZZ_TARGET    ?=
FUZZ_TIME      ?= 60

# Fail with a readable message when a target depends on something a later
# roadmap phase has not created yet.
# $(call require,<path>,<what it is>)
define require
	@if [ ! -e "$(1)" ]; then \
		echo "error: $(2) not set up yet (missing $(1)). See the roadmap." >&2; \
		exit 1; \
	fi
endef

##@ General

.PHONY: help
help: ## Show this help
	@awk 'BEGIN {FS = ":.*##"; printf "Usage: make \033[36m<target>\033[0m\n"} \
		/^[a-zA-Z0-9_-]+:.*?##/ { printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2 } \
		/^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) }' $(MAKEFILE_LIST)

.PHONY: tools
tools: ## Install the Rust dev tools used by other targets
	$(CARGO) install --locked cargo-deny cargo-fuzz
	rustup component add rustfmt clippy

##@ Contributing

DEVTOOLS := $(CARGO) run --quiet --package authloom-devtools --

.PHONY: hooks
hooks: ## Install the git hooks (pre-commit checks, commit message lint, pre-push checks)
	$(DEVTOOLS) install-hooks

.PHONY: branch
branch: ## Create a correctly named branch for an issue (interactive)
	@$(DEVTOOLS) branch

.PHONY: commit
commit: ## Write a Conventional Commit for the staged changes (interactive)
	@$(DEVTOOLS) commit

.PHONY: lint-commits
lint-commits: ## Check this branch's commits against main, as CI does
	$(DEVTOOLS) lint-range origin/main..HEAD

.PHONY: clean
clean: ## Remove build artefacts
	$(CARGO) clean
	rm -rf $(NODE_ADAPTER)/node_modules $(NODE_ADAPTER)/*.node
	rm -rf $(DOCS_SITE)/dist $(DOCS_SITE)/.astro $(DOCS_SITE)/node_modules $(DOCS_SITE)/public/api

##@ Rust

.PHONY: build
build: ## Build the whole workspace
	$(CARGO) build --workspace --all-targets

.PHONY: test
test: ## Run all Rust tests
	$(CARGO) test --workspace --all-targets

.PHONY: fmt
fmt: ## Format all Rust code
	$(CARGO) fmt --all

.PHONY: fmt-check
fmt-check: ## Check formatting without changing files
	$(CARGO) fmt --all --check

.PHONY: lint
lint: ## Run clippy with warnings as errors
	$(CARGO) clippy --workspace --all-targets -- -D warnings

.PHONY: deny
deny: ## Check dependencies for advisories, licences and banned crates
	$(CARGO) deny check

.PHONY: fuzz
fuzz: ## Run a fuzz target: make fuzz FUZZ_TARGET=<name> [FUZZ_TIME=60]
	$(call require,$(FUZZ_DIR),Fuzzing)
	@if [ -z "$(FUZZ_TARGET)" ]; then \
		echo "error: set FUZZ_TARGET. Available targets:" >&2; \
		$(CARGO) +nightly fuzz list >&2; exit 1; \
	fi
	$(CARGO) +nightly fuzz run $(FUZZ_TARGET) -- -max_total_time=$(FUZZ_TIME)

##@ Adapters

.PHONY: adapters
adapters: adapter-node adapter-python ## Build every language adapter

.PHONY: adapter-node
adapter-node: ## Build the Node.js adapter (napi-rs)
	$(call require,$(NODE_ADAPTER)/package.json,Node adapter packaging)
	$(NPM) --prefix $(NODE_ADAPTER) ci
	$(NPM) --prefix $(NODE_ADAPTER) run build

.PHONY: adapter-python
adapter-python: ## Build the Python adapter into the active venv (maturin)
	$(call require,$(PY_ADAPTER)/pyproject.toml,Python adapter packaging)
	cd $(PY_ADAPTER) && maturin develop

.PHONY: conformance
conformance: adapters ## Run the shared adapter conformance suite
	$(call require,$(CONFORMANCE)/Makefile,Conformance suite)
	$(MAKE) -C $(CONFORMANCE) run

##@ Docs

.PHONY: docs-api
docs-api: ## Build the Rust API docs, failing on broken doc links
	RUSTDOCFLAGS="-D warnings" $(CARGO) doc --workspace --no-deps

.PHONY: docs-serve
docs-serve: ## Serve the docsite with live reload; API reference regenerates on source changes
	$(NPM) --prefix $(DOCS_SITE) install
	$(NPM) --prefix $(DOCS_SITE) run dev -- --host $${DOCS_HOST:-127.0.0.1}

.PHONY: docs-build
docs-build: docs-api ## Build the static docsite, including the API reference, into docs/dist
	$(NPM) --prefix $(DOCS_SITE) ci
	$(NPM) --prefix $(DOCS_SITE) run build

.PHONY: docs-preview
docs-preview: ## Serve the built docsite from docs/dist
	$(NPM) --prefix $(DOCS_SITE) run preview -- --host $${DOCS_HOST:-127.0.0.1}

##@ CI

.PHONY: check
check: fmt-check lint test ## Everything a pull request must pass

.PHONY: ci
ci: check deny docs-api ## Full CI run

##@ Docker

.PHONY: docker-build
docker-build: ## Build the dev image
	$(COMPOSE) build dev

.PHONY: docker-shell
docker-shell: ## Open a shell in the dev container
	$(COMPOSE) run --rm dev

.PHONY: docker-check
docker-check: ## Run `make check` inside the dev container
	$(COMPOSE) run --rm dev make check

.PHONY: docker-ci
docker-ci: ## Run the full CI pipeline in a clean image build
	docker build --target ci -t authloom:ci .

.PHONY: docker-docs
docker-docs: ## Serve the docsite from the container on http://localhost:4321
	$(COMPOSE) --profile docs up docs
