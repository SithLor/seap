
//gen_getter_and_setter!(AccumulatorRegister, eax, ax, al);
macro_rules! near_limit {
    ($val:expr) => {
        match std::mem::size_of_val(&$val) {
            1 => ($val as i8) >= i8::MAX - 1,
            2 => ($val as i16) >= i16::MAX - 1,
            4 => ($val as i32) >= i32::MAX - 1,
            8 => ($val as i64) >= i64::MAX - 1,
            _ => panic!("Unsupported type"),
        }
    };
}


fn main() {

    let r1 = 0x00;
    let r2 = 0x01;
    let r3 = 0x02;
    let r4 = 0x03;
    let r5 = 0x04;
    let r6 = 0x05;
    let r7 = 0x06;
    let r8 = 0x07;

    //inst nop,mov,add,sub,mul,div,jump,call,ret,

    matcth 
}
