pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod arr;


pub type WINDOWS_PATH_SEPARATOR = char;
pub type UNIX_PATH_SEPARATOR = char;
pub const WINDOWS_PATH_SEPARATOR: WINDOWS_PATH_SEPARATOR = '\\';
pub const UNIX_PATH_SEPARATOR: char = '/';
pub const HTTP_OK: u16 = 200;
pub const HTTP_NOT_FOUND: u16 = 404;
pub const HTTP_INTERNAL_SERVER_ERROR: u16 = 500;
pub const HTTP_FORBIDDEN: u16 = 403;
pub const HTTP_BAD_REQUEST: u16 = 400;
pub const HTTP_UNAUTHORIZED: u16 = 401;
pub const UNIX_PATH_SEPARATOR: char = '/';
pub const PI: f64 = 3.14159;

pub fn safe_path(arg: &str) -> String {
    //check for / to // or \ to \\
    let mut new_arg = arg.to_string();
    for c in new_arg.chars() {
        if c == '/' {
            new_arg = new_arg.replace("/", "//");
        }
        if c == '\\' {
            new_arg = new_arg.replace("'", "\\'");
        }
    }
    return new_arg;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
