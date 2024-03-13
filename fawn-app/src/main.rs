fn test(){
    println!("custom::nt::peb::PEB_OSBuildNumber:{}",fawn_lib::custom::nt::peb::OSBuildNumber());
    println!("custom::nt::peb::PEB_OSMinorVersion:{}",fawn_lib::custom::nt::peb::OSMinorVersion());
    println!("custom::nt::peb::PEB_OSMajorVersion:{}",fawn_lib::custom::nt::peb::OSMajorVersion());
    println!("custom::nt::peb::PEB_InheritedAddressSpace:{}",fawn_lib::custom::nt::peb::IsInheritedAddressSpace());
    println!("custom::nt::peb::PEB_ImageBaseAddress:0x{:x}",fawn_lib::custom::nt::peb::ImageBaseAddress());
    println!("custom::nt::peb::IsBeingDebugged:{}",fawn_lib::custom::nt::peb::IsBeingDebugged());

}
fn main() {
    test()
}