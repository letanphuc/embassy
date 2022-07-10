/* For STM32F746 */

MEMORY
{
    FLASH : ORIGIN = 0x8000000, LENGTH = 1048576
    RAM : ORIGIN = 0x20000000, LENGTH = 327680
}



/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
