#!/bin/bash

# Use installed tool by default
svd2rust_bin="svd2rust"
# Automates the steps specified in https://docs.rs/svd2rust/0.19.0/svd2rust/
if [ -f svd2rust ]; then
	# If the local directory contains svd2rust, use that version instead
	svd2rust_bin="./svd2rust"
elif [ -f ../svd2rust ]; then
	# Keeps the repository clean
	svd2rust_bin="../svd2rust"
fi
if [ -x "$(${svd2rust_bin} --version)" ]; then
	echo "No svd2rust found locally or installed." \
		"Install it with cargo install svd2rust"
	exit
fi

if ! command -v form &> /dev/null
then
    echo "form tool was not found"
    exit 1
fi

if ! command -v svdtools &> /dev/null
then
    echo "svdtools was not found"
    exit 1
fi

svdtools patch svd/va108xx-patch.yml
# See https://github.com/rust-embedded/svd2rust/issues/830 for required re-export.
${svd2rust_bin} --reexport-interrupt --impl-defmt defmt --impl-debug-feature debug -i svd/va108xx.svd.patched

result=$?
if [ $result -ne 0 ]; then
  echo "svd2rust failed with code $result"
  exit
fi

rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo fmt
