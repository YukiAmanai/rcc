#!/bin/bash

rcc="./target/debug/rcc"
try() {
    expected="$1"
    input="$2"

    cargo run -- "$input" > tmp.s
    gcc -static -o tmp tmp.s
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
try 10 '-10+20'
try 10 '12+(-7)'
try 1 '13==13'
try 1 '12<=12'
try 1 '12 < 13'
try 0 '13< 23'
try 0 '12+(-3)!= 13+(-1*3)'
echo OK
