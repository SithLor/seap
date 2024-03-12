#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
pub mod nt_custom {

    use std::arch::asm;
    use std::os::raw::c_void;

    //pub mod system_error_hex;
    //pub mod system_error_num;
    //pub mod system_error_human;
    //https://learn.microsoft.com/en-us/windows/win32/api/ntdef/ns-ntdef-list_entry
    //https://learn.microsoft.com/en-us/windows/win32/api/ntdef/ns-ntdef-_unicode_string
    //http://undocumented.ntinternals.net/index.html?page=UserMode%2FStructures%2FRTL_DRIVE_LETTER_CURDIR.html
    //https://doxygen.reactos.org/d3/d61/include_2ndk_2pstypes_8h_source.html#l00643

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
    pub type PVOID = *mut c_void;
    pub type PVOID_PTR = *mut PVOID;
    pub type USHORT = u16;
    pub type USHORT_PTR = *mut USHORT;
    pub type ULONG = u32;
    pub type ULONG_PTR = *mut ULONG;
    pub type HANDLE = PVOID;
    pub type BOOLEAN = u8; // BYTE is typically represented as u8 in Rust
    pub type BOOLEAN_PTR = *mut BOOLEAN;
    pub type DWORD = u32;

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
        //unsafe {
        //    asm!("mov rax, qword ptr gs:[0x60]", out("rax") peb);
        //}
        PEB_READ_ASM!(peb);
        unsafe { (*peb).BeingDebugged }
    }

    //HANDLE OpenProcess(
    //    [in] DWORD dwDesiredAccess,
    //    [in] BOOL  bInheritHandle,
    //    [in] DWORD dwProcessId
    //  );

    pub fn PEB_ImageBaseAddress() -> usize {
        let mut peb: PEB_PTR;
        //unsafe {
        //    asm!("mov rax, qword ptr gs:[0x60]", out("rax") peb);
        //}
        PEB_READ_ASM!(peb);
        unsafe { (*peb).ImageBaseAddress as usize }
    }
    pub fn PEB_InheritedAddressSpace() -> bool {
        let mut peb: PEB_PTR;
        //unsafe {
        //    asm!("mov rax, qword ptr gs:[0x60]", out("rax") peb);
        //}
        PEB_READ_ASM!(peb);
        unsafe { (*peb).InheritedAddressSpace }
    }
    pub fn PEB_OSMajorVersion() -> u32 {
        let mut peb: PEB_PTR;
        //unsafe {
        //    asm!("mov rax, qword ptr gs:[0x60]", out("rax") peb);
        //}
        PEB_READ_ASM!(peb);
        unsafe { (*peb).OSMajorVersion }
    }
    pub fn PEB_OSMinorVersion() -> u32 {
        let mut peb: PEB_PTR;
        //unsafe {
        //    asm!("mov rax, qword ptr gs:[0x60]", out("rax") peb);
        //}
        PEB_READ_ASM!(peb);
        unsafe { (*peb).OSMinorVersion }
    }
    pub fn PEB_OSBuildNumber() -> u32 {
        let mut peb: PEB_PTR;
        //unsafe {
        //    asm!("mov rax, qword ptr gs:[0x60]", out("rax") peb);
        //}
        PEB_READ_ASM!(peb);
        unsafe { (*peb).OSBuildNumber }
    }
}
//ms inml of window api very not good for hide stuff
//https://microsoft.github.io/windows-docs-rs/doc/windows/?search=GetUserNAme
//docs
mod win32 {
    pub fn GetUserNameUTF8()->String{
        //pub unsafe fn GetUserNameA(lpbuffer: PSTR, pcbbuffer: *mut u32) -> Result<()>
        //use windows::Win32::System::WindowsProgramming::GetUserNameA;
        use windows::Win32::System::WindowsProgramming::GetUserNameA;
        let mut data;
        let mut name: String;
        unsafe {
            GetUserNameA(data, pcbbuffer)
        }
        name
    }
}
