ENTRY(_start)

/* Where certain sections of a object file in final image */
SECTIONS {

    /* It's recommended to use 1M under BIOS, as it's guaranteed to exist under bios
     * 
     * UEFI strongly suggests 2M is safer.
     * 
     * multiboot2 spec allows us to inform the bootloader within a certain address range
     * 
     * See: https://wiki.osdev.org/Bare_Bones#Bootstrap_Assembly#Linking_the_Kernel  for more info 
     *
    */
    . = 2M;

    	/* First put the multiboot header, as it is required to be put very early
	   in the image or the bootloader won't recognize the file format.
	   Next we'll put the .text section. */
	.text BLOCK(4K) : ALIGN(4K)
	{
		*(.multiboot)
		*(.text)
	}

	/* Read-only data. */
	.rodata BLOCK(4K) : ALIGN(4K)
	{
		*(.rodata)
	}

	/* Read-write data (initialized) */
	.data BLOCK(4K) : ALIGN(4K)
	{
		*(.data)
	}

	/* Read-write data (uninitialized) and stack */
	.bss BLOCK(4K) : ALIGN(4K)
	{
		*(COMMON)
		*(.bss)
	}
}