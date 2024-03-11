APP_RELEASE="./fawn-app/target/x86_64-pc-windows-gnu/debug/fawn-app.exe"
cd fawn-app
cargo build --target x86_64-pc-windows-gnu 
cd ..
wine $APP_RELEASE
