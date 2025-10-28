MEMORY
{
	FLASH : ORIGIN = 0x00011800, LENGTH = 0xE7F8 /* (128k - 12k) / 2 - 8 */
	RAM : ORIGIN = 0x10000000, LENGTH = 0x08000 /* 32K */
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
