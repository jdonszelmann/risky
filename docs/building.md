
# Building the Risky kernel 

The entire build process is supplied by cargo. This means in theory you only need rust installed. However, you will need to install a riscv cross compiler as well to compile a few c and assembly files. This happens automatically through the [build.rs](../build/build.rs) script.

## Installing rust

To compile risky, you will need a very specific version of rust, which can compile for riscv. To install this, use [rustup](https://rustup.rs/). Make sure you have the nightly toolchain installed (`rustup toolchain add nightly`) and run 
```
rustup target add --toolchain nightly riscv64gc-unknown-none-elf
```

## Installing the cross compiler

You will need the riscv cross compiler suite. This can be installed through your favourite package manager:

| distro | package | debugger |
| --- | --- | --- |
| Arch linux (pacman) | [riscv64-unknown-elf-gcc (aur)](https://aur.archlinux.org/packages/riscv64-unknown-elf-gcc/) | [riscv64-unknown-elf-gdb (aur)](https://aur.archlinux.org/packages/riscv64-unknown-elf-gdb/)  |

### Debugger
Optionally you can also install the debugger. Package names are listed as well.


[Alternatively, you can also compile them from source.](https://github.com/riscv/riscv-gnu-toolchain)

# Actually building

To build Risky, run 
```
cargo build +nightly
``` 

A binary will now be created in `target/riscv64gc-unknown-none-elf/debug/risky`. You can now follow the [running](running.md) instructions.
