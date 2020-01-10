#!/bin/sh
set -e
LOGFILE="$PWD/build_mod_$1.log"
MODULE=mod_$1
echo building $MODULE $2, see $LOGFILE
cd $MODULE && cargo build $2 > $LOGFILE 2>&1
