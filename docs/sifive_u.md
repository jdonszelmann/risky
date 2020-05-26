
# Sifive Unleashed

## Boot sequence
* first code executed at 0x1000.
* 5 instructions, setting up the processor:
https://github.com/qemu/qemu/blob/fea8f3ed739536fca027cf56af7f5576f37ef9cd/hw/riscv/spike.c#L217
```
    0x1000      auipc       t0,0x0      // store pc + 0 in t0
    0x1004      addi        a1,t0,32    // add a1 = t0 + 32 
    0x1008      csrr        a0,mhartid  // Read control status register into a0
    0x100c      0x182b283               // I'm not quite sure yet why this loads 0x80000000 in t0
    0x1010      jr  t0                  // jump to t0 (0x80000000)
```
* 

## instructions
| instruction | long name | description |
| --- | --- | --- |
| auipc dst, imm20 | add upper immediate to pc| takes a 20 bit immediate, shifts this left by 12 bits (filling the right with zeroes) and then adds the PC to this, forming a pc-relative address. This is stored in dst. |
| addi dst, src, imm6  | add immediate | adds a 6 bit immediate to src and stores the result in dst |
| li dst, imm[xx] | load immediate | loads an immediate value into dst. This is a pseudo instruction and might expand to multiple instructions.


## CSRs
| register | description |
| --- |--- |
| mhartid | hardware thread id|