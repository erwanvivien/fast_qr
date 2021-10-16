#!/bin/sh

cargo build --release

args="$*"
if [ -z "$args" ]; then
    args="https://vahan.dev/"
fi

echo "$args"

echo "$args" | perf record --call-graph=dwarf target/release/qrgen && \
perf script -F +pid > /tmp/test.perf && \

echo "View results on: https://profiler.firefox.com/"
