
# using input.txt
rust impl 1:4000ms, read as string

rust impl 2:19ms  use buffer

rust impl 3:22ms use rayon per_iter


java impl 1: 770ms 

# using input.m.txt
rust impl 1: 32500ms, read as strings
rust impl 2: 155ms , use buffer
rust impl 3: 155ms , rayon per_iter

java impl 1: 1413ms 


# Commands 

```sh
cd src
java -ea ./main.java
cd ..
cargo build --release
cargo run --release
```

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