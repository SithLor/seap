function build(){
    rm -rf ./BINS/fawn_app/fawn_app.exe
    #build the libary for fawn-lib
    cd fawn-lib
    cargo build --release -j 64
    cd ..

    APP_RELEASE="./fawn-app/target/x86_64-pc-windows-gnu/release/fawn-app.exe"
    cd fawn-app
    cargo build --target x86_64-pc-windows-gnu --release -j 64
    cd ..
    cp $APP_RELEASE ./BINS/fawn_app/fawn_app.exe
}
function test_linux(){
    rm -rf ./BINS/fawn_app/fawn_app.exe
    cd fawn-lib
    cargo build --release -j 64
    cd ..

    APP_RELEASE="./fawn-app/target/x86_64-pc-windows-gnu/debug/fawn-app.exe"
    cd fawn-app
    cargo build --target x86_64-pc-windows-gnu -j 64
    cd ..
    wine $APP_RELEASE
}

# check 
## ask $1 is os type then ask $2 is build type if on wsl open windows terminal
if [ "$1" == "wsl" ]; then
    if [ "$2" == "release" ]; then
        build
    else
        test_linux
    fi
else
    if [ "$2" == "release" ]; then
        build
    else
        test_linux
    fi
fi
