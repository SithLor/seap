#!/bin/bash

# hashes the all file and folder in cwd except for folder name target and file name sig.sh
# using openssl dgst 
# using algo blake2b512,blake2s256,md4,md5,mdc2,rmd160,sha1,sha224,sha256,sha3-224,sha3-256,sha3-384,sha3-512,sha384,sha512,sha512-224,sha512-256,shake128,shake256,sm3, 
# output is in the format of HASH_TYPE FILE_NAME HASH

# check if openssl is installed
#!/bin/bash

# List of algorithms
algos=("blake2b512" "blake2s256" "md4" "md5" "mdc2" "rmd160" "sha1" "sha224" "sha256" "sha3-224" "sha3-256" "sha3-384" "sha3-512" "sha384" "sha512" "sha512-224" "sha512-256" "shake128" "shake256" "sm3")

# Loop through all files in the current directory and its subdirectories
# Ignore any folder called target or .git, and file called sig.sh
find . -type d \( -name 'target' -o -name '.git' \) -prune -o -type f ! -name 'sig.sh' -exec bash -c '
    for file do
        for algo in "${algos[@]}"; do
            hash=$(openssl dgst -$algo "$file" | cut -d "=" -f 2 | xargs)
            echo "$algo $file $hash" >> sig.txt
        done
    done
' bash {} +