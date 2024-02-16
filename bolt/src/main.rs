#[macro_export]
macro_rules! called_from {
    () => {
        format!("/{}/{}:{}",file!(),line!(),column!())
    };
}

//
pub mod filesystem;
//simple file server beamng config 
fn main() {
    println!("Hello, world!");
}
