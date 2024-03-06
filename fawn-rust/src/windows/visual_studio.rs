//pub mod intrinsics {
//    pub mod x86{
//        use crate::windows::types::DWORD;
//
//        pub fn __readfsbyte(offset: u32) -> u8 {
//            let result: u8;
//            unsafe {
//                std::arch::asm!("mov {}, fs:[{}]", out(reg_byte) result, in(reg) offset);
//            }
//            result
//        }
//        pub fn __readfsdword(offset: DWORD) -> DWORD {
//            let result: u32;
//            unsafe {
//                std::arch::asm!("mov {}, fs:[{}]", out(reg) result, in(reg) offset);
//            }
//            result
//        }
//        pub fn __readfsqword(offset: DWORD) -> QWORD {
//            let result: u64;
//            unsafe {
//                std::arch::asm!("mov {}, fs:[{}]", out(reg) result, in(reg) offset);
//            }
//            result
//        }
//        pub fn __readfsword(offset: DWORD) -> u16 {
//            let result: u16;
//            unsafe {
//                std::arch::asm!("mov {}, fs:[{}]", out(reg) result, in(reg) offset);
////            }
////            result
//        }
//    }
//    pub mod x64{
//        use crate::{DWORD, QWORD};
//
//        pub fn __readgsbyte(offset: u32) -> u8 {
//            let result: u8;
//            unsafe {
//                std::arch::asm!("mov {}, gs:[{}]", out(reg_byte) result, in(reg) offset);
//            }
//            result
//        }
//        pub fn __readgsdword(offset: DWORD) -> DWORD {
//            let result: u32;
//            unsafe {
//                std::arch::asm!("mov {}, gs:[{}]", out(reg) result, in(reg) offset);
//            }
//            result
//        }
//        pub fn __readgsqword(offset: DWORD) -> QWORD {
//            let result: u64;
//            unsafe {
//                std::arch::asm!("mov {}, gs:[{}]", out(reg) result, in(reg) offset);
//            }
//            result
//        }
//        pub fn __readgsword(offset: DWORD) -> u16 {
//            let result: u16;
//            unsafe {
//                std::arch::asm!("mov {}, gs:[{}]", out(reg) result, in(reg) offset);
//            }
//            result
//        }
//    }
//}

