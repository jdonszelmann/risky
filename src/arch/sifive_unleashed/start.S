# TODO: start.s may be risc64 generic (with some linker script magic) so may be moved to src/arc/riscv64gc_generic

.section .boot, "ax"
.global entry
entry:
#    UNIMP

    # disable all harts with id > 0
    csrr t0, mhartid
    bnez t0, wfi

    # supervisor access translation and protect register
    # should be zero (disables virtual memory and more)
    #csrr t0, satp
    #csrw satp, zero

    .option push
    .option norelax
    	la gp, __global_pointer$
    .option pop

    la sp, __KERNEL_STACK_POINTER
    mv fp, sp

    # load the position
    la t0, exception
    # and it to mask with ~0b00000011 = ~0x0003 = -0x4
    andi t0, t0, -4
    # or it with the exception mode (0)
    ori t0, t0, 0x0

    csrw mtvec, t0

    la a0, __KERNEL_START
    la a1, __KERNEL_END

    jal zero, sifive_unleashed_kernel_main

.global wfi
wfi:
    wfi
    j wfi


.align 2
exception:
    # REGBYTES defined in build.rs
    # code inspired by https://sifive.cdn.prismic.io/sifive/d1984d2b-c9b9-4c91-8de0-d68a5e64fa0f_sifive-interrupt-cookbook-v1p2.pdf#page=12&zoom=auto,-267,226
    addi sp, sp, -32*REGBYTES

    sw x1, 1*REGBYTES(sp)
    # WARNING: sp is not *exactly* what it was when the interrupt was triggered
    # 32*REGBYTES was subtracted from it to make space for this struct itself.
    sw x2, 2*REGBYTES(sp)
    sw x3, 3*REGBYTES(sp)
    sw x4, 4*REGBYTES(sp)
    sw x5, 5*REGBYTES(sp)
    sw x6, 6*REGBYTES(sp)
    sw x7, 7*REGBYTES(sp)
    sw x8, 8*REGBYTES(sp)
    sw x9, 9*REGBYTES(sp)
    sw x10, 10*REGBYTES(sp)
    sw x11, 11*REGBYTES(sp)
    sw x12, 12*REGBYTES(sp)
    sw x13, 13*REGBYTES(sp)
    sw x14, 14*REGBYTES(sp)
    sw x15, 15*REGBYTES(sp)
    sw x16, 16*REGBYTES(sp)
    sw x17, 17*REGBYTES(sp)
    sw x18, 18*REGBYTES(sp)
    sw x19, 19*REGBYTES(sp)
    sw x20, 20*REGBYTES(sp)
    sw x21, 21*REGBYTES(sp)
    sw x22, 22*REGBYTES(sp)
    sw x23, 23*REGBYTES(sp)
    sw x24, 24*REGBYTES(sp)
    sw x25, 25*REGBYTES(sp)
    sw x26, 26*REGBYTES(sp)
    sw x27, 27*REGBYTES(sp)
    sw x28, 28*REGBYTES(sp)
    sw x29, 29*REGBYTES(sp)
    sw x30, 30*REGBYTES(sp)
    sw x31, 31*REGBYTES(sp)

    # put this Registers struct in a0 (argument 0)
    mv a0, sp

    jal sifive_unleashed_core_local_interrupt_handler

    lw x1, 1*REGBYTES(sp)
    lw x2, 2*REGBYTES(sp)
    lw x3, 3*REGBYTES(sp)
    lw x4, 4*REGBYTES(sp)
    lw x5, 5*REGBYTES(sp)
    lw x6, 6*REGBYTES(sp)
    lw x7, 7*REGBYTES(sp)
    lw x8, 8*REGBYTES(sp)
    lw x9, 9*REGBYTES(sp)
    lw x10, 10*REGBYTES(sp)
    lw x11, 11*REGBYTES(sp)
    lw x12, 12*REGBYTES(sp)
    lw x13, 13*REGBYTES(sp)
    lw x14, 14*REGBYTES(sp)
    lw x15, 15*REGBYTES(sp)
    lw x16, 16*REGBYTES(sp)
    lw x17, 17*REGBYTES(sp)
    lw x18, 18*REGBYTES(sp)
    lw x19, 19*REGBYTES(sp)
    lw x20, 20*REGBYTES(sp)
    lw x21, 21*REGBYTES(sp)
    lw x22, 22*REGBYTES(sp)
    lw x23, 23*REGBYTES(sp)
    lw x24, 24*REGBYTES(sp)
    lw x25, 25*REGBYTES(sp)
    lw x26, 26*REGBYTES(sp)
    lw x27, 27*REGBYTES(sp)
    lw x28, 28*REGBYTES(sp)
    lw x29, 29*REGBYTES(sp)
    lw x30, 30*REGBYTES(sp)
    lw x31, 31*REGBYTES(sp)

    addi sp, sp, 32*REGBYTES

    mret
