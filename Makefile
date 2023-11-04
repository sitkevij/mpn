# http://www.gnu.org/software/make/manual/make.html#Special-Variables
.DEFAULT_GOAL := release

# http://www.gnu.org/software/make/manual/make.html#Phony-Targets
.PHONY: clean docker

TARGET_DIR = target
DEBUG_DIR = $(TARGET_DIR)/debug
RELEASE_DIR = $(TARGET_DIR)/release
RLS_DIR = $(TARGET_DIR)/rls
INSTALL_DIR = /usr/local/bin
BINARY = mpn

all: fmt test clean

fmt:
	cargo fmt --all --verbose

fmt-check:
	cargo fmt --all -- --check

debug:
	export RUSTFLAGS=""
	cargo build

release: test
	cargo build --release

test:
	cargo test --verbose --all -- --nocapture

example:
	cargo run --example simple

dev-install-tools: cargo-install-tools python-install-tools

cargo-install-tools:
	cargo install cargo-bloat
	cargo install cargo-deb
	cargo install cargo-geiger
	cargo install cargo-trend
	cargo install cargo-show
	cargo install cargo-outdated
	cargo install cargo-edit
	cargo install hyperfine
	cargo install --list

python-install-tools:
	pip install codespell

npm-install-tools:
	npm install markdownlint-cli2 --global

publish-dry-run:
	cargo publish --dry-run
	cargo package --list

geiger:
	cargo geiger

tarpaulin:
	# use docker as tarpaulin only supports x86_64 processors running linux
	docker run --security-opt seccomp=unconfined -v "${PWD}:/volume" xd009642/tarpaulin
	open tarpaulin-report.html

grcov:
	# grcov requires rust nightly for now
	rm -rf target/debug/
	# export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off"
	export CARGO_INCREMENTAL=0 && \
	export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" && \
	export RUSTDOCFLAGS="-Cpanic=abort" && \
	cargo +nightly build
	cargo +nightly test --verbose
	grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
	open target/debug/coverage/index.html

install: release debug test
	cargo install --path . 
	## cp $(RELEASE_DIR)/$(BINARY) $(INSTALL_DIR)/$(BINARY)

install-force: clean release debug test
	cargo install --path . --force

clippy:
	cargo clippy

docker-build:
	docker build -t sitkevij/mpn:latest .

docker-run:
	docker run -i sitkevij/mpn:latest

deb:
	cargo deb

manpage:
	target/debug/mpn --help >target/debug/mpn.1.txt
	pandoc MANPAGE.md -s -t man
	HELP=$(cat target/debug/mpn.1.txt)
	echo "$HELP"
	MANPAGE=$(cat MANPAGE.md)
	# echo $MANPAGE | sed 's/$/\\n/g' | tr -d'\n'
	pandoc --standalone --to man MANPAGE.md -o mpn.1
	cp mpn.1 /usr/local/share/man/man1
	man mpn

lint: lint-clippy lint-format lint-markdown lint-spell

lint-clippy:
	cargo clippy --workspace --all-targets --verbose
	cargo clippy --workspace --all-targets --verbose --no-default-features
	cargo clippy --workspace --all-targets --verbose --all-features

lint-format:
	cargo fmt --all -- --check

lint-markdown:
	markdownlint-cli2 --config .markdownlint.json *.md

lint-spell:
	codespell -L crate -w src/*.rs *.md tests/*.rs *.toml

clean: ## Remove all artifacts
	rm -rf $(DEBUG_DIR)
	rm -rf $(RELEASE_DIR)
	rm -rf $(RLS_DIR)
