#build the libary for fawn-lib
cd fawn-lib
cargo build --release
cd ..

APP_RELEASE="./fawn-app/target/x86_64-pc-windows-gnu/release/fawn-app.exe"
cd fawn-app
cargo build --target x86_64-pc-windows-gnu --release
cd ..
cp $APP_RELEASE ./OUT/fawn_app.exe


