# seap
a lib,app for lots ting like marcos and stuff

url for https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
https://godbolt.org/
idea for a program

mini js eninge in rust apps 
for amp

#[macro_export]
macro_rules! called_from {
    () => {
        format!("{}/{}:{}",file!(),line!(),column!())
    };
}
https://samrambles.com/guides/window-hacking-with-rust/creating-a-window-with-rust/index.html#wm_paint
https://godbolt.org/
fast portable util
https://github.com/MicrosoftDocs/win32/blob/docs/desktop-src/Debug/system-error-codes--0-499-.md?plain=1
cargo rustc -C opt-level=3
rustup default nightly
rustup default stable
rustc FILE --emit llvm-bc
rustc FILE --emit mir
rustc FILE --emit asm 
rustc FILE --emit llvm-ir
rustc FILE --emit obj
rustc FILE --emit metadata 
rustc FILE --emit link 
rustc FILE --emit dep-info



# file 
package_app.sh
package_dll.sh
install.sh
debug_system_error.py