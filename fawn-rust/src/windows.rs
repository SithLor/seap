
pub mod system_error_hex;
pub mod system_error_num;
pub mod system_error_human;
pub mod types;
pub mod visual_studio;

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



