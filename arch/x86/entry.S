.att_syntax
// https://www.gnu.org/software/grub/manual/multiboot/multiboot.pdf

.set ALIGN,     1<<0
.set MEMINFO,   1<<1 /* Memory Map*/
.set FLAGS,     ALIGN|MEMINFO
.set MAGIC,     0x1BADB002 /* Magic */

.section .multiboot
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM


/*
 * multiboot standard does not define a value for esp.
 * 
 * This allocates room for a small stack then allocating 16384 bytes
 * 
 * This also allows the stack to be 16 bit aligned
*/
.section .bss
.align 16
stack_bottom:
.skip 16384 // 16 KiB
stack_top:

.section .text
.type _start, @function
.global _start
_start:
    /*
    * We are currently in 32-bit protected mode on x86
    * 
    * Interrupts and paging are disabled
    *
    */
    mov $stack_top, %esp

    // Initialize paging, and segmentation interrupts will be enabled inside
    // kernel_main
    call kernel_main





/*
 * Set the size of the _start symbol to the current location '.' minus its start.
 * This is useful when debugging or when you implement call tracing.
*/
.size _start, . - _start