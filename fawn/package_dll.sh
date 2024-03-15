DLL_RELEASE="./fawn-dll/target/x86_64-pc-windows-gnu/release/fawn_dll.dll"
mkdir FAWN

cd fawn-dll
cargo build --target x86_64-pc-windows-gnu --release
cd ..
cp $DLL_RELEASE ./OUT/fawn_dll.dll
