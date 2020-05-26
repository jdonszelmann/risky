OUTPUT_ARCH( "riscv" )
ENTRY(entry)


MEMORY {
    ram (rwx) : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS {
    __KERNEL_STACK_SIZE = DEFINED(__KERNEL_STACK_SIZE) ? __KERNEL_STACK_SIZE : 2K;

    PROVIDE(__KERNEL_START = .);
    PROVIDE(__KERNEL_CODE_START = .);
    PROVIDE(__KERNEL_BOOT_START = .);

    .boot : {
         KEEP (*(SORT_NONE(.boot)))
    } >ram AT>ram :ram

    PROVIDE(__KERNEL_BOOT_END = .);

    .text : {
        *(.text)
    } >ram AT>ram :ram

    PROVIDE(__KERNEL_CODE_END = .);

    .rodata : {
        *(.rdata)
        *(.rodata .rodata.*)
    } >ram AT>ram :ram

    . = ALIGN(4);

    PROVIDE(__KERNEL_DATA_START = .);

    .data : {
        *(.data)
        /* PROVIDE( __global_pointer$ = . + 0x800 ); */
        *(.sdata .sdata.*)
        *(.gnu.linkonce.s.*)
        . = ALIGN(8);
        *(.srodata.cst16)
        *(.srodata.cst8)
        *(.srodata.cst4)
        *(.srodata.cst2)
        *(.srodata .srodata.*)
    } >ram AT>ram :ram

    PROVIDE(__KERNEL_DATA_END = .);
    PROVIDE(__KERNEL_BSS_START = .);

    . = ALIGN(4);
    .bss : {
        *(.bss)
        *(COMMON)
        . = ALIGN(4);
    } >ram AT>ram :ram

    . = ALIGN(8);
    PROVIDE(__KERNEL_BSS_END = .);
    PROVIDE(__KERNEL_END = .);

    .stack ORIGIN(ram) + LENGTH(ram) - __KERNEL_STACK_SIZE : {
        PROVIDE( __KERNEL_HEAP_END = . );
        . += __KERNEL_STACK_SIZE;
        PROVIDE( __KERNEL_STACK_POINTER = . );
    } >ram AT>ram :ram
}