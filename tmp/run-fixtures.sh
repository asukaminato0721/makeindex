#!/bin/bash
set -euo pipefail
BIN=target/debug/makeindex
TMP=tmp/regress
mkdir -p "$TMP"
run_fixture() {
  local name=$1
  shift
  local idx="makeindex/test/${name}.idx"
  local ind="$TMP/${name}.ind"
  local ilg="$TMP/${name}.ilg"
  "$BIN" -q -o "$ind" -t "$ilg" "$@" "$idx"
  diff -u "makeindex/test/ok-${name}.ind" "$ind"
}

base=(b209a b209b b209c b209d b209e b209f b211a b211d b211e b211f b211g b211h book test tort)
for name in "${base[@]}"; do
  run_fixture "$name"
done
run_fixture b211b -s makeindex/test/b211b.ist -g
run_fixture b210a -s makeindex/test/b210a.ist
run_fixture b211c -s makeindex/test/b211c.ist
