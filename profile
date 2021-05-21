#!/bin/sh
# shortcut to recompile both game_state and mod_rendering, in case of working on rendering innards
export RUST_BACKTRACE=full
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/nvidia_icd.json

set -e
./rebuild-mods && cargo build $1 && heaptrack target/debug/sg_engine

