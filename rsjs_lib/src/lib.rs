pub mod excel;
pub mod file;
pub mod info;
pub mod js_math;
pub mod remote;
pub mod strings;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(kv::get("test"), "test");
    }
}
