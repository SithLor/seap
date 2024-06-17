//shit vm for intel custom i386 with fancy pipeline modern cpu feature expect for branch predtion beascue it allready run a cpu with branch predtion
//https://en.wikipedia.org/wiki/I386

//Pipeline:
//extention:aes,possable support for proteched mode
///info
/// Data width 32 bits
/// Address 32 bits
/// Pipeline
/// 6 stage instruction
mod i386 {
    struct AccumulatorRegister {
        pub eax: i32,
        pub ax: i16,
        pub al: i8,
    }
    struct IndexRegister {
        /// extend Stack pointer 32 bit
        pub esp:i32,
        /// Stack pointer 16 bit
        pub sp:i16,
        /// extend base pointer 32 bit
        pub ebp:i32,
        /// base point 16 bit
        pub bp:i16,
        ///
    }

}

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
//https://www.ardent-tool.com/CPU/docs/Intel/387/datasheets/240225-009.pdf

fn main() {
    //80387SX  math co processor
    let chicken_bit_fpu = 0;


    


}
