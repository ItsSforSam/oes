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
	@echo "Documenting More Niche Functions/types"
	@echo "Please remember these are intended to be limited \
	to a small subset of the kernel's API and are exported \
	primarily to cross crate boundaries"

	RUSTFLAGS += "--cfg document_niche"
	@export RUSTFLAGS
	cargo doc 
clean:
	cargo clean

help:
	@echo "* oes-kernel : Compile the kernel (default)"
	@echo ""
	@echo "* fetch : Fetches kernel dependencies. \
	This will fetch based on the `target` environment variable if defined."
	@echo * fetch-all : Same as `fetch` but fetches for all dependencies
# 	@echo
	@echo "* doc* : Documents kernel's internal api functions, we have several "subcommands" \
	\n ** The normal `doc` target, documents the normal functions and types. It will include dependencies that OES uses. \
	\n ** The `doc-minimal` target documents the bare minimal, excluding dependencies. \
	\n ** The `doc-niche` will include more "niche" functions and types. This is similar to cargo's document-private-items \
	flag, but will not include functions and types which are more implementation details. \
	A "niche" function or type are details that need to expand multiple crates, but shouldn't be called \
	by the average function. Like startup code, kernel_start, etc. Any of these functions should have the 
	same stability requirements as a publicly exported function. Changes to safety requirements should notify necessary people"


.PHONEY: default oes-kernel clean doc-minimal help

