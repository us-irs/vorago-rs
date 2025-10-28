VA416xx Bootloader Application
=======

This is the Rust version of the bootloader supplied by Vorago.

## Memory Map

The bootloader uses the following memory map:

| Address | Notes | Size |
| ------ | ---- |  ---- |
| 0x0 | Bootloader start | code up to 0x3FFC bytes |
| 0x3FFC | Bootloader CRC | word |
| 0x4000 | App image A start | code up to 0x1DFF8 (~120K) bytes |
| 0x21FF8 | App image A CRC check length | word |
| 0x21FFC | App image A CRC check value | word |
| 0x22000 | App image B start | code up to 0x1DFF8 (~120K) bytes |
| 0x3FFF8 | App image B CRC check length | word |
| 0x3FFFC | App image B CRC check value | word |
| 0x40000 | End of NVM | end  |

## Additional Information

As opposed to the Vorago example code, this bootloader assumes a 40 MHz external clock
but does not scale that clock up. It also uses a word (4 bytes) instead of a half-word for the CRC
and uses the ISO 3309 CRC32 standard checksum.

This bootloader does not provide tools to flash the NVM memories by itself. Instead, you can use
the [flashloader](https://egit.irs.uni-stuttgart.de/rust/va416xx-rs/src/branch/main/flashloader)
application to perform this task using a CCSDS interface via a UART.

The bootloader performs the following steps:

1. The application will calculate the checksum of itself if the bootloader CRC is blank (all zeroes
   or all ones). If the CRC is not blank and the checksum check fails, it will immediately boot
   application image A. Otherwise, it proceeds to the next step.
2. Check the checksum of App A. If that checksum is valid, it will boot App A. If not, it will
   proceed to the next step.
3. Check the checksum of App B. If that checksum is valid, it will boot App B. If not, it will
   boot App A as the fallback image.

You could adapt and combine this bootloader with a non-volatile memory to select a prefered app
image, which would be a first step towards an updatable flight software.

Please note that you *MUST* compile the application at slot A and slot B with an appropriate
`memory.x` file where the base address of the `FLASH` was adapted according to the base address
shown in the memory map above. The memory files to do this were provided in the `scripts` folder.
