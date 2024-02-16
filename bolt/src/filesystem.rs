use std::{
    fs::File,
    io::{
        Read,
        Write
    }
};

pub fn ReadFileUTF8(path:&str) -> Result<String, std::io::Error> {
    let mut file: File = File::open(path)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
pub fn WriteFileUTF8(path:&str, contents:&str) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

