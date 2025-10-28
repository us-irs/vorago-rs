#!/bin/bash
# Start the JLinkGDBServer while also specifying the JLinkScript file. The JLinkScript is necessary
# to disable ROM protection to allow flashing
JLinkGDBServer -select USB -device Cortex-M4 -endian little -if SWD -speed 2000 \
  -LocalhostOnly -vd -jlinkscriptfile ./jlink/JLinkSettings.JLinkScript
