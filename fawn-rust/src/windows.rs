
pub mod system_error_hex;
pub mod system_error_num;
pub mod system_error_human;
pub mod types;
pub mod visual_studio;
//rust marco like Path{C:\\My\\Path\\Here} -> "C:\\My\\Path\\Here"


pub fn EcsapePath(path: &str) -> String {
    for i in path.chars(){
        if i == '\\' {
            path.replace("\\", "\\\\");
        }
    }
    return path.to_string();
}

pub fn IsDebuggerPresent() -> bool {}
pub fn GetUserNameA() -> String {
    "user".to_string()
}
pub fn GetUserNameW() -> String {
    "user".to_string()
}
pub fn GetComputerNameA() -> String {
    "user".to_string()
}
pub fn GetComputerNameW() -> String {
    "user".to_string()
}



pub mod TYPES {
    pub type ULONG = u32;
    //VOID* in C
    pub type VOID_PTR = *mut std::ffi::c_void;
}

//https://www.vergiliusproject.com/kernels/x64/Windows%2011
pub mod kernel {
    pub mod window_11 {
        //Windows 11 23H2 (2023 Update)
        //build 10.0_22631.2428
        //2023-10-01
        pub mod _23H2_10_0_22631_2428 {
            //link--https://www.vergiliusproject.com/kernels/x64/Windows%2011/23H2%20(2023%20Update)/Change
            use crate::windows::TYPES::ULONG;
            use crate::windows::TYPES::VOID_PTR;

            #[allow(non_camel_case_types)]
            #[repr(u64)]//this there make it 0x8 instend of 0x4
            pub enum wil_details_ServiceReportingKind {
                wil_details_ServiceReportingKind_UniqueUsage = 0,
                wil_details_ServiceReportingKind_UniqueOpportunity = 1,
                wil_details_ServiceReportingKind_DeviceUsage = 2,
                wil_details_ServiceReportingKind_DeviceOpportunity = 3,
                wil_details_ServiceReportingKind_PotentialUniqueUsage = 4,
                wil_details_ServiceReportingKind_PotentialUniqueOpportunity = 5,
                wil_details_ServiceReportingKind_PotentialDeviceUsage = 6,
                wil_details_ServiceReportingKind_PotentialDeviceOpportunity = 7,
                wil_details_ServiceReportingKind_EnabledTotalDuration = 8,
                wil_details_ServiceReportingKind_EnabledPausedDuration = 9,
                wil_details_ServiceReportingKind_DisabledTotalDuration = 10,
                wil_details_ServiceReportingKind_DisabledPausedDuration = 11,
                wil_details_ServiceReportingKind_CustomEnabledBase = 100,
                wil_details_ServiceReportingKind_CustomDisabledBase = 150,
                wil_details_ServiceReportingKind_Store = 254,
                wil_details_ServiceReportingKind_None = 255,
                wil_details_ServiceReportingKind_VariantDevicePotentialBase = 256,
                wil_details_ServiceReportingKind_VariantDeviceUsageBase = 320,
                wil_details_ServiceReportingKind_VariantUniquePotentialBase = 384,
                wil_details_ServiceReportingKind_VariantUniqueUsageBase = 448,
            }

            //link--https://www.vergiliusproject.com/kernels/x64/Windows%2011/23H2%20(2023%20Update)/CMP_OFFSET_ARRAY
            #[repr(C)]
            pub struct CMP_OFFSET_ARRAY{
                FileOffset:ULONG,//0x0
                DataBuffer:VOID_PTR,//0x8
                DataLength:ULONG//0x10
            }

            pub struct Change {
                kind: wil_details_ServiceReportingKind, 
                count: ULONG, 
            }

            pub struct _HHIVE {}
            pub struct _LIST_ENTRY {}
            pub struct _EX_RUNDOWN_REF {}
            pub struct _CM_KEY_HASH_TABLE_ENTRY {}
            //link--https://www.vergiliusproject.com/kernels/x64/Windows%2011/23H2%20(2023%20Update)/_CMHIVE
        
            #[repr(C)]
            pub struct _CMHIVE {
                Hive: _HHIVE, // 0x0
                FileHandles: [VOID_PTR; 6], 
                NotifyList: _LIST_ENTRY, // 0x638
                HiveList: _LIST_ENTRY, // 0x648
                PreloadedHiveList: _LIST_ENTRY, // 0x658
                HiveRundown: _EX_RUNDOWN_REF, // 0x668
                KcbCacheTable: *mut _CM_KEY_HASH_TABLE_ENTRY, // Assuming the next offset
                KcbCacheTableSize:ULONG                                                //0x678
            }


        }
    }
}