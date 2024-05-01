
# 100k

code_fast(): 97ms
code_faster(): 39ms
code_rayon(): 98ms
code_faster_2(): 11ms

# 1 millons

code_fast(): 4846ms
code_faster(): 2405ms
code_rayon(): 5378ms
code_faster_2(): 1027ms


# Commands 

```sh
cargo rustc --release -- -Z tune-cpu=machine -C opt-level=3 -C overflow-checks=true -C strip=debuginfo -C target-cpu=native
./target/release/fast_seatingchart
```