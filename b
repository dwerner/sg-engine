#!/bin/sh
echo building mod_$1 $2
cd mod_$1 && cargo build $2
