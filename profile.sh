#!/bin/sh

args="${*:-https://vahan.dev/}"
echo "$args"

cargo build --release

echo "$args" | perf record --call-graph=dwarf target/release/fast_qr && \
perf script -F +pid > /tmp/test.perf && \

echo "View results on: https://profiler.firefox.com/"
