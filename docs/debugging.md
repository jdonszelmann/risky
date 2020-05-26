
# Debugging Risky

To debug risky, you will first have to [run risky with qemu](running.md#qemu) and have the [correct debugger installed](building.md#debugger). 

Now you can run qemu with the `-s -S` flags which make qemu wait on boot, and run a remote debug server on `localhost:1234`. To directly debug Risky with cargo, run:

```shell script
export CARGO_TARGET_RISCV64GC_UNKNOWN_NONE_ELF_RUNNER=qemu-system-riscv64 -machine sifive_u -bios none -s -S -nographic -kernel
cargo run +nightly
``` 

Now qemu is waiting for you to attach a debugger. To do this run the following commands from the project root: 
```shell script
riscv64-unknown-elf-gdb -q -x gdb_init
```

Now you can either use `layout asm` and `stepi` to debug, or load the risky binary as symbol file with
```gdb
symbol-file target/riscv64gc-unknown-none-elf/debug/risky
``` 


It is also possible to [debug Risky in clion](clion.md#debugging-risky-in-clion)