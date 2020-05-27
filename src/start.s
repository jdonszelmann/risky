
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

    jal zero, kernel_main

.global wfi
wfi:
    wfi
    j wfi
