pub fn handle_option_string(s: Option<String>) -> String {
    match s {
        Some(s) => s,
        None => "Error".to_string(),
    }
}
pub fn handle_option_bool(s: Option<bool>) -> bool {
    match s {
        Some(s) => s,
        None => false,
    }
}
pub fn ptr_const_i8_to_ptr_mut_u16(ptr: *const i8) -> *mut u16 {
    ptr as *mut u16
}

