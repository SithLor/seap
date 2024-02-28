
# using input.txt

rust impl 1:4000ms, 

rust impl 2:19ms  


java impl 1: 770ms 

# using input.m.txt
rust impl 1: 32500ms, 

rust impl 2: 132ms 

java impl 1: 1413ms 


# Commands 

```sh
cd src
java -ea ./main.java
cd ..
cargo build --release
cargo run --release
```