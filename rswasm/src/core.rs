pub struct Vm {
    
}
impl Vm {
    pub fn new() -> Vm {
        Vm {
            
        }
    }
    fn load(&mut self, _path: &str, _mode: &str) {
        let path: &str = _path;
        let mode: &str = _mode;
        match mode {
            "string" => {
                let path: &str = path.split(".").collect::<Vec<&str>>()[0];
                path.to_string();
            }
            "file" => {
                use std::path::Path;
                let path: &Path = Path::new(_path);
                let filename: &std::ffi::OsStr = path.file_name().unwrap();
                filename.to_str().unwrap().to_string();
            }
            _ => {
                panic!("Invalid mode");
            }
        }

    }
}