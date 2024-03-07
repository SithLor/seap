
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

pub fn IsDebuggerPresent() -> bool {return false;}
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

