use cc;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/start.s,link.ld,src/cstart.c");
    // Use the `cc` crate to build an assembly file and statically link it.
    cc::Build::new()
        .target("riscv64gc-unknown-none-elf")
        .file("src/start.s")
        .compile("start");
    cc::Build::new()
        .target("riscv64gc-unknown-none-elf")
        .file("src/cstart.c")
        .compile("cstart");
}
