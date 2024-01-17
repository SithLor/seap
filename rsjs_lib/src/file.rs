pub fn get_file_name(_path:&str) -> String {
    use std::path::Path;
    let path: &Path = Path::new(_path);
    let filename: &std::ffi::OsStr = path.file_name().unwrap();
    filename.to_str().unwrap().to_string()
}


//static file include macro text files
#[macro_export]
macro_rules! include_text_file {
    ($file:expr) => {
        {
            use std::fs;
            let contents = fs::read_to_string($file)
                .expect("Should have been able to read the file");
            contents
        }
    };
}