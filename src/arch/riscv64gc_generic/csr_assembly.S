
.section .text

.global csrread_mstatus
csrread_mstatus:
    csrr a0, mstatus
    ret

.global csrread_mtval
csrread_mtval:
    csrr a0, mtval
    ret

.global csrread_mepc
csrread_mepc:
    csrr a0, mepc
    ret

.global csrread_mvendorid
csrread_mvendorid:
    csrr a0, mvendorid
    ret

.global csrread_marchid
csrread_marchid:
    csrr a0, marchid
    ret

.global csrread_mcause
csrread_mcause:
    csrr a0, mcause
    ret

.global csrread_satp
csrread_satp:
    csrr a0, satp
    ret

.global csrwrite_satp
csrwrite_satp:
    csrw satp, a0
    ret