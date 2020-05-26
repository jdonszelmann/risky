

# Running Risky

## Qemu

After having followed the [build guide](building.md), running risky inside of qemu should be as simple as running
```shell script
cargo +nightly run
```

It is also possible to [debug Risky in qemu](debugging.md)


### Running manually

To invoke qemu manually after [building](building.md), use:
```shell script
qemu-system-riscv64 -machine sifive_u -bios none -nographic -kernel <risky binary> 
```


# Real hardware

As I don't own a SiFive Unleashed processor at this moment, there currently is no guide yet on how to run Risky on real hardware. Help is welcome! 

