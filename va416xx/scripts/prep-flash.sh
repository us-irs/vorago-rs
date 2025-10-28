#!/bin/bash
# Alternative way to unlock the ROM protection of the VA416XX to allow flashing
gdb-multiarch -q --batch -ex 'source prep-flash.gdb'