#!/bin/bash
set -e

if [ "$RISKY_DEBUG" = "on" ]; then
  echo "Debug mode: waiting for a debugger to attach"
  qemu-system-riscv64 -machine sifive_u -bios none -nographic -s -S -kernel "$1"
  exit 0
elif [ "$RISKY_TEST" = "on" ]; then
  echo "Test mode"
  qemu-system-riscv64 -machine sifive_u -bios none -nographic -kernel "$1" > test.log &
  PID="$!"

  tail -f -n +1 "test.log" | while read line
  do
    if [[ $line == *"Tests complete"* ]]; then
      exit 0
    elif [[ $line == *"Test failed"* ]]; then
      exit 1
    else
      echo "$line"
    fi
  done

  kill "$PID"
  rm test.log

else
  echo "Running $1"
  qemu-system-riscv64 -machine sifive_u -bios none -nographic -kernel "$1"
fi