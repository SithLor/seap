pub mod modules;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyvalue_test() {
        let (key, value) = keyvalue!("key", "value");
        assert_eq!(key, "key");
        assert_eq!(value, "value");
    }
    #[test]
    fn include_text_file_test() {
        let contents = include_text_file!("src/modules/verions.txt");
        assert_eq!(contents, "rust is better c");
    }
    #[test]
    fn console_log_test() {
     //   format!("");
     //   modules::console::log(file!("src/modules/console.rs"),"hello world");
    } 
}
