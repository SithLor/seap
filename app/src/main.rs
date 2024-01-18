use rsjs_lib::file::get_file_extension;
use rsjs_lib::file::PathMode;


fn main() {

    let file = get_file_extension("test.js",PathMode::StringMethod);
    println!("{}",file);
}