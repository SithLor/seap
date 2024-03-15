#!/bin/bash

# Define the URL of the .tbz file
URL="https://github.com/aristocratos/btop/releases/download/v1.3.2/btop-x86_64-linux-musl.tbz"

# Download the file
wget "$URL"

# Extract the contents
tar xvf btop-x86_64-linux-musl.tbz

# Navigate to the extracted directory
mkdir btop-x86_64-linux-musl
cd btop-x86_64-linux-musl
sudo mv btop /usr/local/bin

# Run the software

