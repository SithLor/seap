#linker=x86_64-pc-windows-gnu
#intel
rustc ./src/windows_debug.rs --verbose --emit asm -C opt-level=3 -Cllvm-args=--x86-asm-syntax=intel
#rustc ./src/windows_debug.rs --verbose --emit asm -C opt-level=3 -Cllvm-args=--x86-asm-syntax=att

