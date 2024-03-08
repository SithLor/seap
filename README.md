# seap
a lib,app for lots ting like marcos and stuff

This code not good this
dont be dumb ass run the fawn malware 
i am learn how to write rust for window ,malware 
ONLY

```rust
#[macro_export]
macro_rules! called_from {
    () => {
        format!("{}/{}:{}",file!(),line!(),column!())
    };
}
```

https://samrambles.com/guides/window-hacking-with-rust/creating-a-window-with-rust/index.html#wm_paint

https://godbolt.org/

https://github.com/MicrosoftDocs/win32/blob/docs/desktop-src/Debug/system-error-codes--0-499-.md?plain=1

```sh
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
```




