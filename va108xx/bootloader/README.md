VA108xx Bootloader Application
=======

This is the Rust version of the bootloader supplied by Vorago.

## Memory Map

The bootloader uses the following memory map:

| Address | Notes | Size |
| ------ | ---- |  ---- |
| 0x0 | Bootloader start | code up to 0x2FFE bytes |
| 0x2FFE | Bootloader CRC | half-word |
| 0x3000 | App image A start | code up to 0xE7F4 (~59K) bytes |
| 0x117F4 | App image A CRC check length | word |
| 0x117F8 | App image A CRC check value | word |
| 0x117FC | App image B start | code up to 0xE7F4 (~59K) bytes |
| 0x1FFF0 | App image B CRC check length | word |
| 0x1FFF4 | App image B CRC check value | word |
| 0x1FFF8 | Reserved section, contains boot select parameter | 8 bytes |
| 0x20000 | End of NVM | end  |

## Additional Information

This bootloader was specifically written for the REB1 board, so it assumes a M95M01 ST EEPROM
is used to load the application code. The bootloader will also delay for a configurable amount
of time before booting. This allows to catch the RTT printout, but should probably be disabled
for production firmware.

This bootloader does not provide tools to flash the NVM memory by itself. Instead, you can use
the [flashloader](https://egit.irs.uni-stuttgart.de/rust/va108xx-rs/src/branch/main/flashloader)
application to perform this task using a CCSDS interface via a UART.

The bootloader performs the following steps:

1. The application will calculate the checksum of itself if the bootloader CRC is blank (all zeroes
   or all ones). If the CRC is not blank and the checksum check fails, it will immediately boot
   application image A. Otherwise, it proceeds to the next step.
2. Read the boot slot from a reserved section at the end of the EEPROM. If no valid value is read,
   select boot slot A.
3. Check the checksum of the boot slot. If that checksum is valid, it will boot that slot. If not,
   it will proceed to the next step.
4. Check the checksum of the other slot . If that checksum is valid, it will boot that slot. If
   not, it will boot App A as the fallback image.

In your actual production application, a command to update the preferred boot slot could be exposed
to allow performing software updates in a safe way.

Please note that you *MUST* compile the application at slot A and slot B with an appropriate
`memory.x` file where the base address of the `FLASH` was adapted according to the base address
shown in the memory map above. The memory files to do this were provided in the `scripts` folder.
