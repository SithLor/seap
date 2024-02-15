#[macro_export]
macro_rules! called_from {
    () => {
        format!("/{}/{}:{}",file!(),line!(),column!())
    };
}
//MOD EXPORTS
pub mod platfrom;
pub mod usd;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_called_from(){
        let result = called_from!();
        assert_eq!(result,"/home/runner/work/mango_lib/mango_lib/src/lib.rs:26:5");
    }
    #[test]
    fn test_usd(){

    }
}
