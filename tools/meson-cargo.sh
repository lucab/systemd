#!/bin/sh -eu

if [ "$#" -ne 3 ]; then
   echo "Usage: $0 BUILD_MODE MODULE_NAME PATH_CARGO_TOML"
   exit 1
fi

CARGO=`which cargo`
MODE="$1"
NAME="$2"
MANIFEST=$(realpath "$3")
LIB=$(echo $NAME | tr '-' '_')

$CARGO build -q --${MODE} --manifest-path ${MANIFEST}

cp -a ../src/${NAME}/target/${MODE}/lib${LIB}_rs.a src/${NAME}/lib${LIB}_rs.a
