target ?= x86_64-unknown-none
CargoConfig := .cargo/config.toml
CommonFlags := -C .cargo/config.toml --frozen
DocFlags := --workspace --keep-going $(CommonFlags)
JOBS := $(patsubst -j%,%,$(filter -j%,$(MAKEFLAGS))) 
RUSTFLAGS ?=
default: oes-kernel


oes-kernel:
	@export RUSTFLAGS
	cargo build -p oes-init-start --target=$(target) --features=uefi -C .cargo/config.toml
# 	target/$(target)/debug/oes-init-start

fetch: $(CargoConfig)
	cargo fetch --target $(target)

fetch-all: $(CargoConfig)
	cargo fetch --locked
doc: fetch-all
	cargo doc $(DocFlags)
doc-minimal: fetch-all
	cargo doc $(DocFlags) --no-deps
doc-niche: fetch-all
	@echo 'Documenting More Niche Functions/types'
	@echo 'Please remember these are intended to be limited \
	to a small subset of the kernel's API and are exported \
	primarily to cross crate boundaries'

	RUSTFLAGS += '--cfg document_niche'
	@export RUSTFLAGS
	cargo doc 
clean:
	cargo clean

help:
	@cat ./docs/make-help.txt


.PHONEY: default oes-kernel clean doc-minimal help

