use crate::system_info::{SystemInfo, AnyValue};
use crate::arch::riscv64gc_generic::csr::Csr;

pub fn get_systeminfo() -> SystemInfo {

    let vendor = Csr::read(Csr::MVendorID);
    let arch = Csr::read(Csr::MArchID);


    SystemInfo {
        vendor: if vendor == 0 {
            AnyValue::String("<Unknown>")
        } else {
            AnyValue::Int(vendor)
        },
        architecture: if arch == 0 {
            AnyValue::String("<Unknown>")
        } else {
            AnyValue::Int(arch)
        },
    }
}