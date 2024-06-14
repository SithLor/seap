//shit vm for intel custom i386 with fancy pipeline modern cpu feature expect for branch predtion beascue it allready run a cpu with branch predtion
//https://en.wikipedia.org/wiki/I386

//Pipeline:
//extention:aes,possable support for proteched mode
///info
/// Data width 32 bits 
/// Address 32 bits 
/// Pipeline
/// 6 stage instruction


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

//arg 0:struct name
//arg 1 name flied of arg 0
//arg 2 type flied
macro_rules! create_getter {
    ($struct_name:ident, $($field_name:ident : $field_type:ty),+) => {
        impl $struct_name {
            $(
                gen_func_pub!($field_name,&self,&$field_type,{&self.$field_name})
//                pub fn $field_name(&self) -> &$field_type {
//                    &self.$field_name
//                }
            )+
        }
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
/// main
// Accumulator register
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
//TODO Work MMU


fn main() {
    struct e {
        a:i32,
        b:i16,
        c:i8
    }
    //for gen_geter
    
    println!("Hello, world!");
}
