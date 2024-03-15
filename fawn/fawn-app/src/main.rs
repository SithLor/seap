
fn main() {
    use fawn_lib::dectect::is_amd;
    if is_amd() {
        println!("You are using an AMD CPU");
    } else {
        println!("You are not using an AMD CPU");
    }

}