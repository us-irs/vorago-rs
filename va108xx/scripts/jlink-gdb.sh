#!/bin/bash
JLinkGDBServer -select USB -device VA10820 -endian little -if JTAG -speed auto \
  -LocalhostOnly -jtagconf -1,-1
