pub mod PHNT_MODE_KERNEL {
    pub const PROCESS_TERMINATE: u32 = 0x0001;
    pub const PROCESS_CREATE_THREAD: u32 = 0x0002;
    pub const PROCESS_SET_SESSIONID: u32 = 0x0004;
    pub const PROCESS_VM_OPERATION: u32 = 0x0008;
    pub const PROCESS_VM_READ: u32 = 0x0010;
    pub const PROCESS_VM_WRITE: u32 = 0x0020;
    // pub const PROCESS_DUP_HANDLE: u32 = 0x0040;
    pub const PROCESS_CREATE_PROCESS: u32 = 0x0080;
    pub const PROCESS_SET_QUOTA: u32 = 0x0100;
    pub const PROCESS_SET_INFORMATION: u32 = 0x0200;
    pub const PROCESS_QUERY_INFORMATION: u32 = 0x0400;
    pub const PROCESS_SET_PORT: u32 = 0x0800;
    pub const PROCESS_SUSPEND_RESUME: u32 = 0x0800;
    pub const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
}

pub const PROCESS_PRIORITY_CLASS_UNKNOWN: u32 = 0;
pub const PROCESS_PRIORITY_CLASS_IDLE: u32 = 1;
pub const PROCESS_PRIORITY_CLASS_NORMAL: u32 = 2;
pub const PROCESS_PRIORITY_CLASS_HIGH: u32 = 3;
pub const PROCESS_PRIORITY_CLASS_REALTIME: u32 = 4;
pub const PROCESS_PRIORITY_CLASS_BELOW_NORMAL: u32 = 5;
pub const PROCESS_PRIORITY_CLASS_ABOVE_NORMAL: u32 = 6;
