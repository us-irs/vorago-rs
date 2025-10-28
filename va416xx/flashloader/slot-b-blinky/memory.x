/* Special linker script for application slot B with an offset at address 0x22000 */
MEMORY
{
	FLASH : ORIGIN = 0x00022000, LENGTH = 256K
	/* RAM is a mandatory region. This RAM refers to the SRAM_0 */
	RAM : ORIGIN = 0x1FFF8000, LENGTH = 32K
	SRAM_1 : ORIGIN = 0x20000000, LENGTH = 32K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
/* SRAM_0 can be used for all busses: Instruction, Data and System */
/* SRAM_1 only supports the system bus */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/* Define sections for placing symbols into the extra memory regions above.   */
/* This makes them accessible from code.                                      */
SECTIONS {
  .sram1 (NOLOAD) : ALIGN(8) {
    *(.sram1 .sram1.*);
    . = ALIGN(4);
    } > SRAM_1
};
