#!/bin/bash

# Check if binary path was provided
if [ "$#" -ne 1 ]; then
  echo "Usage: $0 <path-to-binary>"
  exit 1
fi

BINARY="$1"

# Check if file exists
if [ ! -f "$BINARY" ]; then
  echo "Error: File '$BINARY' not found."
  exit 1
fi

# Run the command
telnet localhost 19021 | defmt-print -e "$BINARY"
