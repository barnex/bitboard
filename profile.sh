#! /bin/bash
set -e

rm -f out.perf perf.data perf.data.old
RUSTFLAGS='-C force-frame-pointers=y -C target-cpu=native' cargo build --bin bench --release
perf record -F997 --call-graph dwarf ./target/release/bench $@
perf script > out.perf
perf report -g graph,caller
