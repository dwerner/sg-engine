#!/bin/sh

LOGFILE="$PWD/build_mod_$1.log"
MODULE=mod_$1
echo "building (opts: $2) $MODULE, see $LOGFILE"
if (cd $MODULE && cargo build $2 > $LOGFILE 2>&1) ; then
    echo building $MODULE succeeded
    exit 0
else 
    echo "FAILED building $MODULE"
    exit 1
fi

