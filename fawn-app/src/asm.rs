//
pub fn crash() {
    unsafe {
        use std::arch::asm;
        asm!("ud2");
    }
}
pub fn INT3_TRAP() {
    unsafe {
        use std::arch::asm;
        asm!("int3");
    }
}