target ?= x86_64-unknown-uefi
default: oes-kernel


oes-kernel:
	cargo build -p oes-init-start --target=$(target) --features=uefi --verbose
	target/$(target)/debug/oes-init-start

.PHONEY: default oes-kernel