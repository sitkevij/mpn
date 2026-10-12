# Makefile for MPN (MPEG-4 Inspector)
# http://www.gnu.org/software/make/manual/make.html#Special-Variables
.DEFAULT_GOAL := help

# http://www.gnu.org/software/make/manual/make.html#Phony-Targets
.PHONY: help all build debug release test test-unit test-integration test-all \
	clean fmt fmt-check clippy lint install uninstall \
	docker-build docker-run docker-push \
	docs doc-open bench benchmark \
	check outdated update-deps \
	ci pre-commit publish-dry-run \
	manpage deb

# Configuration variables
TARGET_DIR = target
DEBUG_DIR = $(TARGET_DIR)/debug
RELEASE_DIR = $(TARGET_DIR)/release
RLS_DIR = $(TARGET_DIR)/rls
INSTALL_DIR = /usr/local/bin
BINARY = mpn
CARGO = cargo
RUSTFLAGS ?=

##@ Help

help: ## Display this help message
	@echo "MPN (MPEG-4 Inspector) - Available Commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'
	@echo ""

##@ Building

all: fmt clippy test release ## Run fmt, clippy, tests, and build release

build: ## Build the project in debug mode
	$(CARGO) build

debug: build ## Alias for build (debug mode)

release: ## Build optimized release binary
	$(CARGO) build --release

release-fast: ## Build release binary with faster compilation (less optimization)
	$(CARGO) build --release --profile release-fast 2>/dev/null || $(CARGO) build --release

##@ Testing

test: ## Run all tests (unit and integration)
	$(CARGO) test -- --test-threads=1

test-unit: ## Run only unit tests
	$(CARGO) test --lib

test-integration: ## Run only integration tests
	$(CARGO) test --test integration_test

test-all: ## Run all tests with verbose output and no capture
	$(CARGO) test --verbose --all -- --nocapture

test-watch: ## Run tests in watch mode (requires cargo-watch)
	@command -v cargo-watch >/dev/null 2>&1 || { echo "Install cargo-watch: cargo install cargo-watch"; exit 1; }
	cargo watch -x test

test-coverage: ## Generate test coverage report (requires cargo-tarpaulin)
	@command -v cargo-tarpaulin >/dev/null 2>&1 || { echo "Install cargo-tarpaulin: cargo install cargo-tarpaulin"; exit 1; }
	cargo tarpaulin --out Html --output-dir ./coverage
	@echo "Coverage report generated in ./coverage/tarpaulin-report.html"

##@ Code Quality

fmt: ## Format all Rust code
	$(CARGO) fmt --all --verbose

fmt-check: ## Check code formatting without making changes
	$(CARGO) fmt --all -- --check

clippy: ## Run clippy linter
	$(CARGO) clippy --all-targets -- -D warnings

clippy-fix: ## Run clippy and automatically apply fixes
	$(CARGO) clippy --fix --all-targets --allow-dirty --allow-staged

lint: lint-clippy lint-format lint-markdown lint-spell ## Run all linters

lint-clippy: ## Run clippy with all feature combinations
	$(CARGO) clippy --workspace --all-targets --verbose
	$(CARGO) clippy --workspace --all-targets --verbose --no-default-features
	$(CARGO) clippy --workspace --all-targets --verbose --all-features

lint-format: fmt-check ## Check code formatting

lint-markdown: ## Lint markdown files
	@command -v markdownlint-cli2 >/dev/null 2>&1 || { echo "Install markdownlint-cli2: npm install -g markdownlint-cli2"; exit 1; }
	markdownlint-cli2 --config .markdownlint.json *.md

lint-spell: ## Check spelling in source files
	@command -v codespell >/dev/null 2>&1 || { echo "Install codespell: pip install codespell"; exit 1; }
	codespell -L crate -w src/*.rs *.md tests/*.rs *.toml

check: fmt-check clippy test ## Run format check, clippy, and tests (CI-friendly)

##@ Documentation

docs: ## Generate documentation
	$(CARGO) doc --no-deps --open

doc: docs ## Alias for docs

doc-open: ## Generate and open documentation in browser
	$(CARGO) doc --no-deps --open

doc-serve: ## Serve documentation locally (requires cargo-doc-server or similar)
	@echo "Documentation available at: file://$(PWD)/target/doc/mpn/index.html"

##@ Benchmarking

bench: benchmark ## Run benchmarks
	$(CARGO) bench

benchmark: ## Run benchmarks (alias for bench)
	$(CARGO) bench

bench-hyperfine: ## Run hyperfine benchmark on CLI (requires hyperfine)
	@command -v hyperfine >/dev/null 2>&1 || { echo "Install hyperfine: cargo install hyperfine"; exit 1; }
	@if [ -f "tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4" ]; then \
		hyperfine --warmup 2 --min-runs 3 "./target/release/$(BINARY) tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4"; \
	else \
		echo "Test file not found. Run tests first to download test files."; \
	fi

##@ Dependencies

outdated: ## Check for outdated dependencies
	@command -v cargo-outdated >/dev/null 2>&1 || { echo "Install cargo-outdated: cargo install cargo-outdated"; exit 1; }
	cargo outdated

update-deps: ## Update all dependencies in Cargo.toml
	@command -v cargo-edit >/dev/null 2>&1 || { echo "Install cargo-edit: cargo install cargo-edit"; exit 1; }
	cargo update

update-deps-interactive: ## Interactively update dependencies
	@command -v cargo-edit >/dev/null 2>&1 || { echo "Install cargo-edit: cargo install cargo-edit"; exit 1; }
	cargo upgrade --interactive

audit: ## Audit dependencies for security vulnerabilities
	@command -v cargo-audit >/dev/null 2>&1 || { echo "Install cargo-audit: cargo install cargo-audit"; exit 1; }
	cargo audit

##@ Development Tools

dev-install-tools: cargo-install-tools python-install-tools npm-install-tools ## Install all development tools

cargo-install-tools: ## Install useful cargo tools
	cargo install cargo-bloat      # Analyze binary size
	cargo install cargo-deb         # Create Debian packages
	cargo install cargo-geiger      # Check unsafe code usage
	cargo install cargo-trend       # Dependency trends
	cargo install cargo-show        # Show crate information
	cargo install cargo-outdated    # Check outdated dependencies
	cargo install cargo-edit        # Edit Cargo.toml
	cargo install cargo-audit       # Security audit
	cargo install cargo-tarpaulin   # Code coverage
	cargo install cargo-watch       # Watch for changes
	cargo install cargo-doc         # Documentation tools
	cargo install hyperfine         # Benchmarking
	cargo install --list

python-install-tools: ## Install Python-based tools
	pip install codespell

npm-install-tools: ## Install npm-based tools
	npm install -g markdownlint-cli2

bloat: ## Analyze binary size (requires cargo-bloat)
	@command -v cargo-bloat >/dev/null 2>&1 || { echo "Install cargo-bloat: cargo install cargo-bloat"; exit 1; }
	cargo bloat --release

geiger: ## Check unsafe code usage (requires cargo-geiger)
	@command -v cargo-geiger >/dev/null 2>&1 || { echo "Install cargo-geiger: cargo install cargo-geiger"; exit 1; }
	cargo geiger

##@ Installation

install: release ## Install the binary to system (via cargo install)
	$(CARGO) install --path .

install-force: clean release ## Force reinstall the binary
	$(CARGO) install --path . --force

install-system: release ## Install binary to system directory (requires sudo)
	@echo "Installing $(BINARY) to $(INSTALL_DIR)"
	sudo cp $(RELEASE_DIR)/$(BINARY) $(INSTALL_DIR)/$(BINARY)
	sudo chmod +x $(INSTALL_DIR)/$(BINARY)

uninstall: ## Uninstall the binary from system
	$(CARGO) uninstall $(BINARY) || rm -f $(INSTALL_DIR)/$(BINARY)
	@echo "Uninstalled $(BINARY)"

##@ Docker

docker-build: ## Build Docker image
	docker build -t sitkevij/$(BINARY):latest .

docker-run: ## Run Docker container interactively
	docker run -it sitkevij/$(BINARY):latest

docker-push: docker-build ## Build and push Docker image
	docker push sitkevij/$(BINARY):latest

docker-test: ## Run tests in Docker container
	docker run --rm -v "$(PWD):/volume" -w /volume rust:latest cargo test

##@ Packaging

deb: ## Create Debian package (requires cargo-deb)
	@command -v cargo-deb >/dev/null 2>&1 || { echo "Install cargo-deb: cargo install cargo-deb"; exit 1; }
	cargo deb

publish-dry-run: ## Dry run of publishing to crates.io
	$(CARGO) publish --dry-run
	$(CARGO) package --list

publish: check ## Publish to crates.io (runs check first)
	$(CARGO) publish

##@ CI/CD

ci: check test-integration ## Run CI checks (format, clippy, tests)

pre-commit: fmt clippy test ## Run pre-commit checks

ci-full: lint test-all ## Run full CI suite (all linters and tests)

##@ Manual Pages

manpage: ## Generate and install man page (requires pandoc)
	@command -v pandoc >/dev/null 2>&1 || { echo "Install pandoc: brew install pandoc (macOS) or apt-get install pandoc (Linux)"; exit 1; }
	@mkdir -p target/debug
	@$(CARGO) build
	@target/debug/$(BINARY) --help >target/debug/$(BINARY).1.txt
	@pandoc --standalone --to man MANPAGE.md -o $(BINARY).1
	@echo "Man page generated: $(BINARY).1"
	@echo "To install: sudo cp $(BINARY).1 /usr/local/share/man/man1/ && sudo mandb"

manpage-install: manpage ## Generate and install man page to system
	@sudo cp $(BINARY).1 /usr/local/share/man/man1/ || echo "Failed to install man page (may need sudo)"
	@sudo mandb 2>/dev/null || echo "mandb not available, man page may still work"
	@man $(BINARY)

##@ Code Coverage

tarpaulin: ## Generate coverage with tarpaulin (Docker version for cross-platform)
	@echo "Using Docker for tarpaulin (supports x86_64 Linux)"
	docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin
	@if [ -f "tarpaulin-report.html" ]; then \
		open tarpaulin-report.html 2>/dev/null || xdg-open tarpaulin-report.html 2>/dev/null || echo "Open tarpaulin-report.html in your browser"; \
	fi

grcov: ## Generate coverage with grcov (requires nightly Rust)
	@echo "grcov requires Rust nightly"
	@command -v grcov >/dev/null 2>&1 || { echo "Install grcov: cargo install grcov"; exit 1; }
	rm -rf target/debug/
	export CARGO_INCREMENTAL=0 && \
	export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" && \
	export RUSTDOCFLAGS="-Cpanic=abort" && \
	cargo +nightly build
	cargo +nightly test --verbose
	grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
	@if [ -f "target/debug/coverage/index.html" ]; then \
		open target/debug/coverage/index.html 2>/dev/null || xdg-open target/debug/coverage/index.html 2>/dev/null || echo "Open target/debug/coverage/index.html in your browser"; \
	fi

##@ Utilities

version: ## Show version information
	@echo "Package: $(BINARY)"
	@echo "Version: $$(grep '^version' Cargo.toml | cut -d'"' -f2)"
	@echo "Rust version: $$(rustc --version)"
	@echo "Cargo version: $$(cargo --version)"

run: build ## Build and run with example file (if available)
	@if [ -f "tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4" ]; then \
		$(CARGO) run -- tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4; \
	else \
		echo "Usage: make run FILE=path/to/file.mp4"; \
		if [ -n "$(FILE)" ]; then \
			$(CARGO) run -- $(FILE); \
		fi \
	fi

run-release: release ## Run release binary with example file
	@if [ -f "tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4" ]; then \
		./target/release/$(BINARY) tests/files/test-bokeh-au-2t-vd-30f-854x480.mp4; \
	else \
		echo "Usage: make run-release FILE=path/to/file.mp4"; \
		if [ -n "$(FILE)" ]; then \
			./target/release/$(BINARY) $(FILE); \
		fi \
	fi

##@ Cleaning

clean: ## Remove all build artifacts
	$(CARGO) clean
	rm -rf $(DEBUG_DIR)
	rm -rf $(RELEASE_DIR)
	rm -rf $(RLS_DIR)
	rm -f $(BINARY).1
	rm -rf coverage
	rm -f tarpaulin-report.html

clean-all: clean ## Remove all artifacts including Cargo registry cache
	$(CARGO) clean --release
	rm -rf .cargo
	rm -rf target/doc

distclean: clean-all ## Remove everything including dependencies (nuclear option)
	@echo "This will remove all Cargo artifacts. Continue? [y/N]"
	@read -r answer && [ "$$answer" = "y" ] && $(CARGO) clean || echo "Cancelled"
