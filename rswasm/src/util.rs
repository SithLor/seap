#[macro_export]
macro_rules! called_from {
    () => {
        {
            let string: String = format!("(/{}/{}:{}:{})",env!("CARGO_PKG_NAME"),file!(),line!(),column!());
            string
        }
    };
}
#[inline(always)]
pub fn call_from() -> String {

    let string: String = format!("(/{}/{}:{}:{})",env!("CARGO_PKG_NAME"),file!(),line!(),column!());
    string
}