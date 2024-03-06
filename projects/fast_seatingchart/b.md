
# 100k

rust impl 1:19916ms

rust impl 2: 41ms 45163.6% INCURE OF speed

rust impl 3: 48ms

rust impl 4:20ms

java impl 1:696ms

# 1 millons

rust impl 1:1200000ms or minutes

rust impl 2: 409ms buffer

rust impl 3:488ms rayon

rust impl 4:195ms buffer

java impl 1:2349ms


# Commands 

```sh
cd src
java -ea ./main.java
cd ..
cargo build --release
cargo run --release
```