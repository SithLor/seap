
//https://www.crow.rip/crows-nest/mal/dev/inject/syscalls/indirect-syscalls
//https://www.nirsoft.net/kernel_struct/vista/TEB.html
//https://klezvirus.github.io/RedTeaming/AV_Evasion/NoSysWhisper/
//https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/262627d8-3418-4627-9218-4ffe110850b2
//https://lenarn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-
//https://www.vergiliusproject.com/kernels/x64/Windows%2010%20%7C%202016/2110%2021H2%20(November%202021%20Update)/_OBJECT_ATTRIBUTES
//https://www.vergiliusproject.com/kernels/x64/Windows%2011
//https://www.geoffchappell.com/studies/windows/km/ntoskrnl/inc/api/pebteb/peb/index.htm
//http://undocumented.ntinternals.net/index.html?page=UserMode%2FStructures%2FRTL_DRIVE_LETTER_CURDIR.html

//https://www.virustotal.com/gui/file/ef1d16f11b184f9017bef21db833313c427f1e10dd0adf37b879c2e2f71548d5/behavior


mod win;
fn main() {
    //let e: bool = windows_api::IsBeingDebugged();
    //println!("ImageBaseAddress:0x{:x}", windows::PEB_ImageBaseAddress());
    //println!("ImageBaseAddress:{}" ,windows::PEB_ImageBaseAddress());
    //println!("OSMajorVersion:{}", windows::PEB_OSMajorVersion());
    //println!("OSMinorVersion:{}", windows::PEB_OSMinorVersion());
    //println!("OSBuildNumber:{}", windows::PEB_OSBuildNumber());
    //println!("InheritedAddressSpace:{}", windows::PEB_InheritedAddressSpace());
    //println!("BeingDebugged:{}", e);
    println!("Hello, world!");
    let e: bool = win::ms_InitiateSystemShutdownA();
    println!("d")
    //panic!("crash");
}