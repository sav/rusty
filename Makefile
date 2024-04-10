EXAMPLES := env1 error1 error2 error3 error4 error5 error6

all: check lint test run-debug echo

update:
	cargo update

debug:
	cargo build

check:
	cargo check

run-debug:
	cargo run

run:
	cargo run --release

test:
	cargo test

.PHONY: echo
echo:
	cargo build --package echo

%: examples/%.rs
	@echo "~~~ $< ~~~"
	@RUST_BACKTRACE=full cargo run --example $@ || true

run-examples: $(EXAMPLES) /tmp/hello.txt

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: fmt-check
fmt-check:
	cargo fmt --check

.PHONY: examples
examples:
	cargo build --examples

lint:
	cargo clippy

lint-fix:
	cargo clippy --fix

lint-all:
	cargo clippy --all-targets --all-features -- -D warnings

fix:
	cargo fix

/tmp/hello.txt:
	@echo ${USER} >> /tmp/hello.txt

clean:
	cargo clean

doc:
	cargo doc --open

##
## The commands below are merely illustrative.
##

modules-tree-bin:
	cargo modules generate tree --with-types --bin rusty

modules-tree-lib:
	cargo modules generate tree --with-types --libs

./target/debug/rusty:
	make build

size: ./target/debug/rusty
	rust-size ./target/debug/rusty
	cargo size --release -- -A -x

## For more about cargo-binutils:
##  https://crates.io/crates/cargo-binutils

binutils-nm:
	cargo nm --release
	cargo nm --release -- --print-size --size-sort

binutils-objdump:
	cargo objdump --release -- --disassemble --no-show-raw-insn

.PHONY:
	test update build debug run lint lint-fix lint-all \
	check modules-tree-bin modules-tree-lib clean doc \
	size binutils-nm binutils-objdump
