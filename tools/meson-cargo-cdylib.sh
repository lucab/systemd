#!/bin/sh
# Mandatory input arguments:
#  $1 - top build directory
#  $2 - cdylib soname

set -e

PROFILE="release"

cargo build \
  --quiet \
  --release \
  --target-dir "$1"

mkdir -p "$1/src/rust/rshared_ffi/"

cp -a "$1/${PROFILE}/librshared_ffi.so" "$1/src/rust/rshared_ffi/$2.so"

patchelf --set-soname "$2" "$1/src/rust/rshared_ffi/$2.so"
