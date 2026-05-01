target ?= x86_64-unknown-uefi
default: oes-kernel


oes-kernel:
	cargo build -p oes-init-start --target=$(target) --features=uefi
	target/$(target)/debug/oes-init-start

clean:
	cargo clean

.PHONEY: default oes-kernel clean

