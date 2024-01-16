#[macro_export]
macro_rules! keyvalue {
    ($key:expr, $value:expr) => {
        {
            let key = $key;
            let value = $value;
            (key, value)
        }
    };
}



//static file include macro text files
#[macro_export]
macro_rules! include_text_file {
    ($file:expr) => {
        {
            use std::fs;
            let contents = fs::read_to_string($file)
                .expect("Should have been able to read the file");
            contents
        }
    };
}


#[macro_export]
macro_rules! u8_to_u16 {
    ($u8:expr) => {
        {
            let u8 = $u8;
            let u16 = u8 as u16;
            u16
        }
    };
}
#[macro_export]
macro_rules! u8_to_u32 {
    ($u8:expr) => {
        {
            let u8 = $u8;
            let u32 = u8 as u32;
            u32
        }
    };
}
#[macro_export]
macro_rules! u8_to_u64 {
    ($u8:expr) => {
        {
            let u8 = $u8;
            let u64 = u8 as u64;
            u64
        }
    };
}
#[macro_export]
macro_rules! i8_to_i16 {
    ($i8:expr) => {
        {
            let i8 = $i8;
            let i16 = i8 as i16;
            i16
        }
    };
}
#[macro_export]
macro_rules! i8_to_i32 {
    ($i8:expr) => {
        {
            let i8 = $i8;
            let i32 = i8 as i32;
            i32
        }
    };
}
#[macro_export]
macro_rules! i8_to_i64 {
    ($i8:expr) => {
        {
            let i8 = $i8;
            let i64 = i8 as i64;
            i64
        }
    };
}
#[macro_export]
macro_rules! u16_to_u32 {
    ($u16:expr) => {
        {
            let u16 = $u16;
            let u32 = u16 as u32;
            u32
        }
    };
}
#[macro_export]
macro_rules! u16_to_u64 {
    ($u16:expr) => {
        {
            let u16 = $u16;
            let u64 = u16 as u64;
            u64
        }
    };
}



