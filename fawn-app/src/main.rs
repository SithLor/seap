
fn test(){

    use fawn_lib::custom::nt::peb::ProcessParameters;
    let e: *mut fawn_lib::custom::nt::peb::RTL_USER_PROCESS_PARAMETERS = ProcessParameters();
    println!("{:?}", e);




}
fn main() {
    test()
}