cd fawn-lib
cargo build --release -j 64
cd ..

APP_RELEASE="./fawn-app/target/x86_64-pc-windows-gnu/debug/fawn-app.exe"
cd fawn-app
cargo build --target x86_64-pc-windows-gnu -j 64
cd ..
wine $APP_RELEASE
