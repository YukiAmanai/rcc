#!/bin/bash

rcc="./target/debug/rcc"
try() {
    expected="$1"
    input="$2"

    cargo run -- "$input" > tmp.s
    cc -o tmp tmp.s
    ./tmp
    actual="$?"

if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

try 0 0
try 42 42
try 90 '(12 + 3) * 6'
try 8 '4 + 8 / 2'
try 47 '5+6*7'
try 15 '5*(9-6)'
try 4 '(3+5)/2'
echo OK
