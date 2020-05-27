

# Adding a new Serial driver to Risky

To add a new serial driver to risky, create a new file for it in `risky/serial/arch`, and add it to the `mod.rs` there. 
A general implementation is given in `risky/serial/arch/sifive_unleashed.rs`. Specific implementations may deviate, but 
the simpelest way to define a new serial driver is to create a zero-sized struct implementing the SerialWriter trait.

To start using the driver, add a variant corresponding to your driver to `risky/serial/mod.rs:Serial`, and optionally add it 
to the `Default` implementation. Make sure to gate your variant (and default) behind the target the driver works for.  