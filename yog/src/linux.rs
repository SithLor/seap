pub fn has_selinux() -> bool {
    let BINARY_PATH: &str = "/system/bin/getenforce";
    let BINARY_PATH_EXIST: bool = std::path::Path::new(BINARY_PATH).exists();
}
