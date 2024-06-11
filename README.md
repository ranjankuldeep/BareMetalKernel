# Building Custom OS for target x86_64 architecute.
Reference Article https://os.phil-opp.com/

## Target
   "arch": "x86_64",
   "llvm-target": "x86_64-unknown-none",

## Prerequiste
1. Cargo Version (1.78.0-nightly).
2. QEMU should be available on the system.

### Build the Kernel
```
    cargo build
```
### Available Tests
```
    cargo test --test basic_boot
    cargo test --test should_panic.rs
    cargo test --test stack_overflow.rs
```
