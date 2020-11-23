
# How Risky boots
 
The first code that is executed on boot, is a short bit of assembly in 
[start.s](../src/arch/sifive_unleashed/start.S). Unfortunately this is necessary. 
Rust just can't function without it. This file is bound to change, 
and the docs might not always update. But I will try to explain what it does 
in the current state as best I can.

It all starts with a section declaration. I put the boot code in a seperate section so it can be put at the start of the kernel by the linker.

```asm
.section .boot, "ax"
```

This entry point will be entered by all Harts in parallel. Currently I don't want to deal with multiple hardware threads, 
so I 'park' most of them in a loop waiting for interrupts. Only the hart with id 0 will continue the boot sequence.
```asm
.global entry
entry:

    # disable all harts with id > 0
    csrr t0, mhartid
    bnez t0, wfi
```

Here, the global pointer is set. This is done to make memory accesses more efficient. [this](https://gnu-mcu-eclipse.github.io/arch/riscv/programmer/) 
(bottom of the page) is a good source explaining exactly what it does.   
```asm
    .option push
    .option norelax
    	la gp, __global_pointer$
    .option pop
```

Before Rust can work, there must be a stack. The stack pointer is initialized to 0 on boot, so it has to be moved. 
The linker script defines a symbol (__KERNEL_STACK_POINTER) where the stack is located. The frame pointer starts out 
at the same place as the stack pointer, meaning the stack is empty at this point
```asm
    la sp, __KERNEL_STACK_POINTER
    mv fp, sp
```

Now the main rust kernel can be called. If the rust main function ever returns, the hart will 
loop and wake on interrupts, just like all other harts did earlier. In practise the rust main function
will never return, and instead just call this function directly at the end of it's code.
```asm
    jal zero, kernel_main

.global wfi
wfi:
    wfi
    j wfi

```
 


