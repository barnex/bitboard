#! /bin/bash
set -e

RUSTFLAGS='-C force-frame-pointers=y -C target-cpu=native' cargo build --bin cli --release
perf record -F997 --call-graph dwarf ./target/release/cli
perf script > out.perf
#perf report -g graph,caller