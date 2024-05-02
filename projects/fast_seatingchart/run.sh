cargo rustc --release -- -Z tune-cpu=machine -C opt-level=3 -C overflow-checks=true -C strip=debuginfo -C target-cpu=native
./target/release/fast_seatingchart > bench.txt

# append to bench.txt

