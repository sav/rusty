## Makefile, Copyright (C) 2024, Savio Sena <savio.sena@gmail.com>

all: build test doc examples

update:
	cargo update

.PHONY: build
build: echo examples
	cargo build

check:
	cargo check

run-debug:
	cargo run

run:
	cargo run --release

.PHONY: test
test:
	cargo test

.PHONY: echo
echo:
	cargo build --package echo

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: fmt-check
fmt-check:
	cargo fmt --check

.PHONY: examples ex
examples:
	cargo build --examples
ex: examples

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

.PHONY: doc
doc:
	cargo doc --all --document-private-items
	cargo doc --package echo --target-dir=target/doc/echo
	cargo doc --examples --target-dir=target/doc/examples --document-private-items

## The commands below are merely illustrative.

modules-tree-bin:
	cargo modules generate tree --with-types --bin rusty

modules-tree-lib:
	cargo modules generate tree --with-types --libs

./target/debug/rusty:
	make build

size: ./target/debug/rusty
	rust-size ./target/debug/rusty
	cargo size --release -- -A -x

count:
	@find echo/ examples/ src/ tests/ -iname '*.rs' | xargs wc -l | tail -1 | cut -f2 -d' '

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
