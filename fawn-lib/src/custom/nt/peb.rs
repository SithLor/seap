#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
use std::arch::asm;
use std::os::raw::c_void;
//import this type
use super::super::types::*;

#[repr(C)]
pub struct UNICODE_STRING {
    Length: USHORT,
    MaximumLength: USHORT,
    Buffer: USHORT_PTR, // PWSTR is a pointer to a wide string, which is represented as *mut u16 in Rust
}
#[repr(C)]
pub struct LIST_ENTRY {
    Flink: *mut LIST_ENTRY,
    Blink: *mut LIST_ENTRY,
}
#[repr(C)]
pub struct RTL_DRIVE_LETTER_CURDIR {
    Flags: USHORT,
    Length: USHORT,
    TimeStamp: ULONG,
    DosPath: UNICODE_STRING,
}
#[repr(C)]
pub struct RTL_USER_PROCESS_PARAMETERS {
    MaximumLength: ULONG,
    Length: ULONG,
    Flags: ULONG,
    DebugFlags: ULONG,
    ConsoleHandle: PVOID,
    ConsoleFlags: ULONG,
    StdInputHandle: HANDLE,
    StdOutputHandle: HANDLE,
    StdErrorHandle: HANDLE,
    CurrentDirectoryPath: UNICODE_STRING,
    CurrentDirectoryHandle: HANDLE,
    DllPath: UNICODE_STRING,
    ImagePathName: UNICODE_STRING,
    CommandLine: UNICODE_STRING,
    Environment: PVOID,
    StartingPositionLeft: ULONG,
    StartingPositionTop: ULONG,
    Width: ULONG,
    Height: ULONG,
    CharWidth: ULONG,
    CharHeight: ULONG,
    ConsoleTextAttributes: ULONG,
    WindowFlags: ULONG,
    ShowWindowFlags: ULONG,
    WindowTitle: UNICODE_STRING,
    DesktopName: UNICODE_STRING,
    ShellInfo: UNICODE_STRING,
    RuntimeData: UNICODE_STRING,
    DLCurrentDirectory: [RTL_DRIVE_LETTER_CURDIR; 0x20],
}
#[repr(C)]
pub struct PEB_LDR_DATA {
    Length: ULONG,
    Initialized: BOOLEAN,
    SsHandle: PVOID,
    InLoadOrderModuleList: LIST_ENTRY,
    InMemoryOrderModuleList: LIST_ENTRY,
    InInitializationOrderModuleList: LIST_ENTRY,
}
#[repr(C)]
pub struct PEB_FREE_BLOCK {
    Next: *mut PEB_FREE_BLOCK,
    Size: ULONG,
}
#[repr(C)]
pub struct PEB {
    InheritedAddressSpace: bool,
    ReadImageFileExecOptions: bool,
    BeingDebugged: bool,
    Spare: bool,
    Mutant: PVOID,
    ImageBaseAddress: PVOID,
    LoaderData: *mut PEB_LDR_DATA_PTR,
    ProcessParameters: RTL_USER_PROCESS_PARAMETERS_PTR,
    SubSystemData: PVOID,
    ProcessHeap: PVOID,
    FastPebLock: PVOID,
    FastPebLockRoutine: *mut PEBLOCKROUTINE_PTR,
    FastPebUnlockRoutine: *mut PEBLOCKROUTINE_PTR,
    EnvironmentUpdateCount: ULONG,
    KernelCallbackTable: PVOID_PTR,
    EventLogSection: PVOID,
    EventLog: PVOID,
    FreeList: *mut PEB_FREE_BLOCK_PTR,
    TlsExpansionCounter: ULONG,
    TlsBitmap: PVOID,
    TlsBitmapBits: [ULONG; 2],
    ReadOnlySharedMemoryBase: PVOID,
    ReadOnlySharedMemoryHeap: PVOID,
    ReadOnlyStaticServerData: PVOID,
    AnsiCodePageData: PVOID,
    OemCodePageData: PVOID,
    UnicodeCaseTableData: PVOID,
    NumberOfProcessors: ULONG,
    NtGlobalFlag: ULONG,
    Spare2: [u8; 4],
    CriticalSectionTimeout: i64,
    HeapSegmentReserve: ULONG,
    HeapSegmentCommit: ULONG,
    HeapDeCommitTotalFreeThreshold: ULONG,
    HeapDeCommitFreeBlockThreshold: ULONG,
    NumberOfHeaps: ULONG,
    MaximumNumberOfHeaps: ULONG,
    ProcessHeaps: PVOID_PTR,
    GdiSharedHandleTable: PVOID,
    ProcessStarterHelper: PVOID,
    GdiDCAttributeList: PVOID,
    LoaderLock: PVOID,
    OSMajorVersion: ULONG,
    OSMinorVersion: ULONG,
    OSBuildNumber: ULONG,
    OSPlatformId: ULONG,
    ImageSubSystem: ULONG,
    ImageSubSystemMajorVersion: ULONG,
    ImageSubSystemMinorVersion: ULONG,
    GdiHandleBuffer: [ULONG; 34],
    PostProcessInitRoutine: ULONG,
    TlsExpansionBitmap: ULONG,
    TlsExpansionBitmapBits: [u8; 128],
    SessionId: ULONG,
}

//_PTR is pointer to the struct
pub type PEB_PTR = *mut PEB;
pub type RTL_DRIVE_LETTER_CURDIR_PTR = *mut RTL_DRIVE_LETTER_CURDIR;
pub type RTL_USER_PROCESS_PARAMETERS_PTR = *mut RTL_USER_PROCESS_PARAMETERS;
pub type PUNICODE_STRING = *mut UNICODE_STRING;
pub type LIST_ENTRY_PTR = *mut LIST_ENTRY;
pub type RLIST_ENTRY_PTR = *mut LIST_ENTRY;
pub type PEB_LDR_DATA_PTR = *mut PEB_LDR_DATA;
pub type PEB_FREE_BLOCK_PTR = *mut PEB_FREE_BLOCK;
pub type PEBLOCKROUTINE_PTR = unsafe fn(*mut c_void);


//this Code crash ON linux
macro_rules! PEB_READ_ASM {
    ($peb:expr) => {
        unsafe {
            asm!("mov rax, qword ptr gs:[0x60]", out("rax") $peb);
        }
    };
}
pub fn IsBeingDebugged() -> bool {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).BeingDebugged }
}
pub fn ImageBaseAddress() -> usize {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ImageBaseAddress as usize }
}
pub fn IsInheritedAddressSpace() -> bool {
    let mut peb: PEB_PTR;

    PEB_READ_ASM!(peb);
    unsafe { (*peb).InheritedAddressSpace }
}


pub fn HeapDeCommitTotalFreeThreshold() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).HeapDeCommitTotalFreeThreshold }
}
pub fn HeapDeCommitFreeBlockThreshold() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).HeapDeCommitFreeBlockThreshold }
}
pub fn NumberOfHeaps() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).NumberOfHeaps }
}
pub fn MaximumNumberOfHeaps() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).MaximumNumberOfHeaps }
}

fn ProcessHeaps() -> PVOID_PTR {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ProcessHeaps }
}
fn GdiSharedHandleTable() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).GdiSharedHandleTable }
}
fn ProcessStarterHelper() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ProcessStarterHelper }
}
fn GdiDCAttributeList() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).GdiDCAttributeList }
}
fn LoaderLock() -> PVOID {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).LoaderLock }
}

pub fn OSMajorVersion() -> u32 {
    let mut peb: PEB_PTR;

    PEB_READ_ASM!(peb);
    unsafe { (*peb).OSMajorVersion }
}
pub fn OSMinorVersion() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).OSMinorVersion }
}
pub fn OSBuildNumber() -> u32 {
    let mut peb: PEB_PTR;
    //unsafe {
    //    asm!("mov rax, qword ptr gs:[0x60]", out("rax") peb);
    //}
    PEB_READ_ASM!(peb);
    unsafe { (*peb).OSBuildNumber }
}
pub fn OSPlatformId() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).OSPlatformId }
}
pub fn ImageSubSystem() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ImageSubSystem }
}
pub fn ImageSubSystemMajorVersion() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ImageSubSystemMajorVersion }
}
pub fn ImageSubSystemMinorVersion() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).ImageSubSystemMinorVersion }
}
fn GdiHandleBuffer(){}
pub fn PostProcessInitRoutine() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).PostProcessInitRoutine }
}
pub fn TlsExpansionBitmap() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).TlsExpansionBitmap }
}
fn TlsExpansionBitmapBits() -> [u8; 128] {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).TlsExpansionBitmapBits }
}
pub fn SessionId() -> u32 {
    let mut peb: PEB_PTR;
    PEB_READ_ASM!(peb);
    unsafe { (*peb).SessionId }
}