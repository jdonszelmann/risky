use crate::interrupt::arch::plic::Plic;

#[repr(usize)]
enum Interrupts {
    L2Cache1 = 1,
    L2Cache2 = 2,
    L2Cache3 = 3,

    Uart0 = 4,
    Uart1 = 5,

    // QSPI 2 (moved down)

    GPIO0 = 7,
    GPIO1 = 8,
    GPIO2 = 9,
    GPIO3 = 10,
    GPIO4 = 11,
    GPIO5 = 12,
    GPIO6 = 13,
    GPIO7 = 14,
    GPIO8 = 15,
    GPIO9 = 16,
    GPIO10 = 17,
    GPIO11 = 18,
    GPIO12 = 19,
    GPIO13 = 20,
    GPIO14 = 21,
    GPIO15 = 22,

    DMA0 = 23,
    DMA1 = 24,
    DMA2 = 26,
    DMA3 = 27,
    DMA4 = 28,
    DMA5 = 29,
    DMA6 = 30,

    DDR = 31,

    CHIPLINK0 = 32,
    CHIPLINK1 = 33,
    CHIPLINK2 = 34,
    CHIPLINK3 = 35,
    CHIPLINK4 = 36,
    CHIPLINK5 = 37,
    CHIPLINK6 = 38,
    CHIPLINK7 = 39,
    CHIPLINK8 = 40,
    CHIPLINK9 = 41,

    PWM00 = 42,
    PWM01 = 43,
    PWM02 = 44,
    PWM03 = 45,

    PWM10 = 46,
    PWM11 = 47,
    PWM12 = 48,
    PWM13 = 49,

    I2C = 50,

    QSPI0 = 51,
    QSPI1 = 52,
    QSPI2 = 6,

    ETHERNET = 53
}

