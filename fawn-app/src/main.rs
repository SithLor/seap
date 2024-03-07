
//https://www.crow.rip/crows-nest/mal/dev/inject/syscalls/indirect-syscalls
//https://www.nirsoft.net/kernel_struct/vista/TEB.html
//https://klezvirus.github.io/RedTeaming/AV_Evasion/NoSysWhisper/
//https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/262627d8-3418-4627-9218-4ffe110850b2
//https://lenarn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-
//https://www.vergiliusproject.com/kernels/x64/Windows%2010%20%7C%202016/2110%2021H2%20(November%202021%20Update)/_OBJECT_ATTRIBUTES
//https://www.vergiliusproject.com/kernels/x64/Windows%2011
//https://www.geoffchappell.com/studies/windows/km/ntoskrnl/inc/api/pebteb/peb/index.htm


use std::arch::asm;

use std::os::raw::{c_void, c_uchar};

#[repr(C)]
struct RTL_USER_PROCESS_PARAMETERS {
    // Define the fields of the RTL_USER_PROCESS_PARAMETERS structure here
}

#[repr(C)]
struct PTEB {
    _reserved: [u8; 0x02], // Reserve space for the fields before BeingDebugged
    BeingDebugged: c_uchar,
    _reserved2: [u8; 0x1D], // Reserve space for the fields between BeingDebugged and ProcessParameters
    ProcessParameters: *const RTL_USER_PROCESS_PARAMETERS,
    // Define the rest of the fields of the TEB here
}

fn main() {
    let offset: i32 = 0x60;
    let teb: *const PTEB;
    unsafe {
        asm!(
            "mov {0}, gs:[{1}]",
            out(reg) teb,
            in(reg) offset,
        );
        // Now `teb` is a pointer to the TEB
        // You can access the BeingDebugged field like this:
        let being_debugged = (*teb).BeingDebugged;
        println!("BeingDebugged: {}", being_debugged);
    }
}