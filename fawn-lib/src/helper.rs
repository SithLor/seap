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


