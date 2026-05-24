target ?= x86_64-unknown-none
CommonFlags := --frozen
DocFlags := --workspace --keep-going $(CommonFlags)
JOBS := $(patsubst -j%,%,$(filter -j%,$(MAKEFLAGS))) 
RUSTFLAGS ?=
default: oes-kernel


oes-kernel:
	@export RUSTFLAGS
	cargo build -p oes-init-start --target=$(target)
# 	target/$(target)/debug/oes-init-start

fetch:
	cargo fetch --target $(target)

fetch-all:
	cargo fetch --locked
doc: fetch-all
	cargo doc $(DocFlags)
doc-minimal: fetch-all
	cargo doc $(DocFlags) --no-deps
doc-niche : RUSTFLAGS += --cfg document_niche
doc-niche: fetch-all
	@echo "Documenting More Niche Functions/types"
	@echo "Please remember these are intended to be limited"
	@echo "to a small subset of the kernel's API and are exported"
	@echo "primarily to cross crate boundaries"
	export RUSTFLAGS
	cargo doc $(DocFlags)
clean:
	cargo clean
clean-doc:
	@echo "Cleaning generated documentation"
	cargo clean --doc
help:
	@cat ./docs/make-help.txt

test:
	cargo test -p oes-kernel-core oes-kernel-core

.PHONEY: default oes-kernel clean doc-minimal help test clean-doc





