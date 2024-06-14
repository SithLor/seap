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
        pub eax:i32,
        pub ax:i16,
        pub al:i8
    }
    impl AccumulatorRegister {
        fn new() -> AccumulatorRegister {
            return AccumulatorRegister {
                eax:0,
                ax:0,
                al:0
            }
        }
    }
}

pub mod intel_sha_e {
    pub mod sha1 {
        pub const SHA1RNDS4:&str = "SHA1RNDS4";
        pub const SHA1NEXTE:&str = "SHA1NEXTE";
        pub const SHA1MSG1:&str = "SHA1MSG1";
        pub const SHA1MSG2:&str = "SHA1MSG2";
    }
    pub mod sha256 {
        pub const SHA256RNDS2:&str = "SHA256RNDS2";
        pub const SHA256MSG1:&str = "SHA256MSG1";
        pub const SHA256MSG2:&str = "SHA256MSG2";
    
        
    }
}


macro_rules! gen_func_pub {
    ($name:ident, $arg:tt, $return_type:ty, $body:block) => {
        pub fn $name $arg -> $return_type $body
    };
}
macro_rules! gen_func_priv {
    ($name:ident, $arg:tt, $return_type:ty, $body:block) => {
        fn $name $arg -> $return_type $body
    };
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


fn main() {




}
