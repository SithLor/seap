curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
##https://github.com/RazrFalcon/cargo-bloat
cargo install cargo-bloat
##https://rustwasm.github.io/twiggy/install.html
cargo install twiggy
##https://github.com/johnthagen/min-sized-rust?tab=readme-ov-file
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
wget https://github.com/sharkdp/hyperfine/releases/download/v1.16.1/hyperfine_1.16.1_amd64.deb
sudo dpkg -i hyperfine_1.16.1_amd64.deb
curl -fsSL https://bun.sh/install | bash


curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
pip install -r requirements.txt
rustup target add x86_64-pc-windows-gnu
sudo apt-get install gcc-mingw-w64-x86-64 -y
