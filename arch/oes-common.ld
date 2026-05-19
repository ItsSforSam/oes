/* Linker script agnostic of the architecture */

/*Stolen from https://docs.rs/unwinding/0.2.8/unwinding/#baremetal */
. = ALIGN(8);
.eh_frame : { KEEP (*(.eh_frame)) *(.eh_frame.*) }
