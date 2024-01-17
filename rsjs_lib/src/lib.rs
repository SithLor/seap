pub mod kv;
pub mod file;
pub mod info;
pub mod js_math;
pub mod js_crypto;
pub mod strings;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(kv::get("test"), "test");
    }
}
