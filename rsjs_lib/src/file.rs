pub enum NameMode {
    StringMethod,
    FileSystem,
}
pub fn get_file_name(_path:&str,mode:NameMode) -> String {
    match mode {
        NameMode::StringMethod => {
            let path: &str = _path;
            let path: &str = path.split(".").collect::<Vec<&str>>()[0];
            path.to_string();
        }
        NameMode::FileSystem => {
            use std::path::Path;
            let path: &Path = Path::new(_path);
            let filename: &std::ffi::OsStr = path.file_name().unwrap();
            filename.to_str().unwrap().to_string();
        }
        
    }
    use std::path::Path;
    let path: &Path = Path::new(_path);
    let filename: &std::ffi::OsStr = path.file_name().unwrap();
    filename.to_str().unwrap().to_string()
}

pub enum PathMode {
    StringMethod,
    FileSystem,
}
pub fn get_file_extension(_path:&str,mode:PathMode) -> String {
    match mode {
        PathMode::StringMethod => {
            let path: &str = _path;
            let path: &str = path.split(".").collect::<Vec<&str>>()[1];
            path.to_string()
        }
        PathMode::FileSystem => {
            use std::path::Path;
            let path: &Path = Path::new(_path);
            let extension: &std::ffi::OsStr = path.extension().unwrap();
            return extension.to_str().unwrap().to_string();
        }
    }
}

//static file include macro text files
#[macro_export]
macro_rules! get_file_extension {
    ($_path:expr) => {
        {
            let path: &str = _path;
            let path: &str = path.split(".").collect::<Vec<&str>>()[1]
            path.to_string()
        }
    };
}

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
//when called prints line number and file name // file!(),line!(),column!() codeonline
