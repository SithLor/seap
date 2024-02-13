#[macro_export]
macro_rules! called_from {
    () => {
        format!("/{}/{}:{}",file!(),line!(),column!())
    };
}

pub mod windows;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
