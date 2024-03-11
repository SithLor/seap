# seap
A library and application for various things like macros and more.

**WARNING:** This project involves the use of NTAPI (Windows Native API), which can be unstable and potentially risky. Please use caution when running or modifying this code. We are not responsible for any damage or issues that may arise from its use.

## Installation
```sh 
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Python dependencies
pip install -r requirements.txt

# Add Rust target for Windows
rustup target add x86_64-pc-windows-gnu
cargo install rustfilt

# Install GCC for Windows
sudo apt-get install gcc-mingw-w64-x86-64 -y
sudo apt-get install wine
# Make scripts executable
chmod +x ./package_app.sh
chmod +x ./package_dll.sh



# Debug Commands 
```sh 
cargo rustc -C opt-level=3
rustup default nightly
rustup default stable
rustc FILE --emit llvm-bc
rustc FILE --emit mir
rustc FILE --emit asm 
rustc FILE --emit llvm-ir
rustc FILE --emit obj
rustc FILE --emit metadata 
rustc FILE --emit link 
rustc FILE --emit dep-info
```

# Nice Rust Macro 
```rust 
#[macro_export]
macro_rules! called_from {
    () => {
        format!("{}/{}:{}",file!(),line!(),column!())
    };
}
```
https://rust.godbolt.org/#z:OYLghAFBqd5QCxAYwPYBMCmBRdBLAF1QCcAaPECAMzwBtMA7AQwFtMQByARg9KtQYEAysib0QXACx8BBAKoBnTAAUAHpwAMvAFYTStJg1DEArgoKkl9ZATwDKjdAGFUtEywYhJAJlKOAMngMmABy7gBGmMQSABzSAA6oCoR2DC5uHl6%2Bicm2AoHBYSyR0VxxlpjWeQxCBEzEBOnunj4VVam19QQFoRFRsdLmXU2ZrUMNPUUlAwCUlqgmxMjsHGaYANTm6CAg9chIuwosAKQAzABCxxoAgmubBNsgSTvETADuO8gA%2BgBuqHjoM6XG5Xa4AejB8RM4XWLAwmwAnuZMCwvlFiCQvghMKogaCIVCYXD0Ijkaj0ZiGO48TcCdDYfCFEiCCi0cQMcQse5DDTwWCEAQCPEFCAIfR6gwAHQsPDIDEKVBUAiStAsMGMAC0ZjBbyC6FQbwUOqCp28YKY8TwYMEWCo1oUGptmCoGtoeHMaMExAR%2BP5guForB4uIUplcqSiuVqvVDC1Rt1DH1huNDFN5st1oezvtjqzLq%2BJgYsowmC%2B5mIQWAvoFQsDhf1yHcjBZ6ElgiCLJDYgUbcwBDBepxkoFLFoZwAYvEmMBMGcACKKKIAWRLx28AFZvOPaqYbItMAo15vxwAlAAq/i%2Bc5PAEkAGrYL7%2BbBns/YE9fJxyE9zm8n4cEKO1b%2BiKEL6qoCIzlKxCYEwNhJJKJDAGC6CnChABsXADgwyBuFgXzeImADWBHCgQCLxAeXwxAgZYLEsmAAUB3inLQGgaOhkinKCa7ceu5wMKgXwsIYwD0Mc65zjxLESecMHxMQEBODMElSTchL3LuBDrHIIQ3k4ADyc6PkIZ63iEADi6zHAA7MC1zrI56z%2BIwwAEEgOlCAAEgZ56kKCTnrEuTCqHgLDuC5RjuSAnk%2BX5AVOecJhUFQ/Sxb5Z5fMoZmkOsELrMoADqpknus7rrEw6yJB2UTrEQFXrLqWCaZWuVvAgsoIGVCjrPJMFKIImAkkwPUAFThdpJhcOhZUMOsJ5mAQPE2Wp1y8bJgnCaJ4mSdJfFyZgClKSpu3qfS5YmDYzk3qZXzYCEZkAJrWXZCWOeOboMERMXjSY2n%2BDdmX3U9/k3IF5yfd96y/f9gN3Q9J6PaDa0rXtG1CSJRg7at638X1x2qaCGkXVd56Xte96Ps%2Br7vp%2B36/qVtn2YFH3TiK6XxWDTmRW5HlyN5GXI4FZ5hZgnQsPEMVyP4BmWULTlzkkyhMNFOl6YZxlfCVN5y8tOMyfxm2Y2Js6nWtBsHUdymE2dMIk9pZNfPztPKCeBlONgQhCFl1wntcS4vu%2BQgvczTnBaF4UsDzqvS7LFny450d8zLutc%2B9BjAOzsepw5CuYOEJjAKzmdSyn8dvesLgMAq9BeYY6DiAVd4GTec4J5XAg15gxdZ2X7e1OgN4MFCBB14mjdedcIRzs%2B/cPAZf0j2PDfsOsk/T7PFcD9g7IkMvE9TzP2Dt04iwwYIc54DB8HesrMfq0ZJlmTr5dp5XZ/Npf19EN6%2B%2Br%2BvR925zloLQO%2BfMH6a21jnQKN4RIzjASEVgq9dL6UflrZ%2B0CnIuBYJjdAPQpYQKfuZV%2BudHLYAYD8K%2BAg2CCBisoZurc55dErMoJIKR8jOgIKXOOTCGgsLYdUM8qBJY6T7hXQqAIY5iLfl5TAeBgACm4ZgxyTgED1AkegKRPCK6qPqLI%2BRijRHaLflXLuZ4cQEGuIKCsBcWS92MaQ9YEjEwGh7kokhgUhAIANM4pMbijHKKcXqA0IsCCNxQRrIhL8gEHiIkQeIiC2AENQZAjBHinJeMqLQIe/BkmRPQcQ9uC12xsDnCrJgeS0FQPSY5Gep92SfyvpgG%2BCIYqyUdhTB8T4Xxvg/F%2BH8f4gTrA0KobwGhVLI1svrfaRttqm2mbJfG1szbEwIFpAq2BzhPmvFea4Z4HJMwrkndx7ch4pDEHgAAXkNGK5wDIGWfFPfuCg/50IYW3CuQ9/CoCYOgAyxAsDEBXOgEw9BAjmBigDW6wNEanIYEuFEJAET/MBcC0FmBwVcOutChGSNPkMDObYC5lyVapBRcuDA6LMWQrhjCvFIJUYggtrMrG8y0Z40OopZZq1VnrOUJsr4p5sCPnODLJwABpEOFcQgWJ%2BhNDZWyhUirFeK/uVzkHSJRgsw2GM5k23NvtJZJ0eXnTWZdbS/LzhSrfkPbEFYWzXHQOgfqCghBTmWDFcIqBXBFNgoPOB3c6A4FUM0gy8Rqjsy9T6iu5w5FGDnPnQuM5tjrCjbQfuU4YKeu9emiuS4/qGCxfQluHybUBvOCNTAjrnUHnZsWxhRyfmArKXUOVf0FXbI/HOPZ1wso5QrsoDEywFAKGVq8NgnZ2aO2dh%2BV27tPbe2UL7f2gcTyLv7W/IQ0IhDMhRC2ipTcS3t0HagYdLzYIiPraWxx44RoEGUPnb5yAoZXvbre8wD7whPqIieBYthghtotZslVbs5Bnh1o%2BbKJ4313s/XIBgtBT0/r/UEVeMMFUgYMmBiDfboMV3IZQjEDAaHyHiOgFWmAXCFixdnGp6xxVRGCLQJwYhaDhDgkRM8TBwiNyvbh9u2AfjNm%2BcAIQzTqhvKPfhoTggROSYbW/ccMEMXuixehy1gqTzCq%2BKK924r%2BMVzPLQBQ2BVBTmrqkKjg1ogBLo0ZhQ5xCAiUve89u9nHOAQtB59mslaNDO8BMiuJ4/UGQQwiLx9QhoIrhN6CtSh5PXsCsF35oXaDhbUTBdA0WkWyItAl31KWwudFsMgMTxAhPEH3fliu1wLMuCwMrGcVXX0VwMiierKhpyYCq4ehTjj4PFiwCxpQXGePdfKdVt%2BUwogGSoCes9JB7GBJCAQCyiH2O0GLicrembMDeDafxEwMQhmSECyY%2B1soxBiZsKkEWbA/0xTwJxduuX4hieACR4LShysaocYFV772SPYJlDRzV/2L0JuB4QIRdRNvKbPAgfq3jaAptoy9iHlHUA4MIEpzAmAIZIYR0j1wqOwdOWm8QWbr2lt0fDmFdwFOqcXpp8eodtbqcJYM2/Cy%2BAIuZb/qN3jrmB1s5HZ0BoURZG0EojZlr3P8BzicFYtZeBbEqYhb1xL3Mm1RG/ZNxxBkhDBW0CQO8URkj2Fs%2B3Q3S4gim/N6kbbb9DdJToOgCnTuDdCGUAYAg/BiAsBvKTv7TlYFda3eEHdZJPcwIDRHqPLIWDG/t8QC3ngrefLj9u3dSe7fEDN6nx3Gf5d4D/klFKaVfNlyGacU7klj1JHvaLhQhLf1/VQzHpy9nTPmbTx55znfHLd7M4YPvTmvOEB84d47Fx1hcG8Md%2BvW9a2pCD57qZPEQS0jBLh7qVV/jWbqqgOq2JNLmuskymZurWX6o0uRSiHaoPWVOHOaG8rLW8lxgJa/Jtb/0nvxsB0reF0tTL0nTAMrOmZM/q/uhkAZTN0jTH0vTIMhcOyt/ltDfisv/hRIAReE7EIC7G7B7F7D7H7AHL0uuozC/m/u2tOgQbOkQQuqQSuhQZ/syj/tjETNgQ/soBElUmktATQdpHwaksQmwVfhgb/lgTCAAdikDLirvvOEIXIfDE9OIejJIZwbbHVDgfNFCvIU9IodQehvoaoYjOoTqpoWytobIRpv4Dst2vskYTAe/gKvYV2j2hYegcbFodcHfroRpkqjpiqs4coYEVpsqnpl4SylISajIQEcBnpqBuBiEJBlAUoYWAoEwKlOsFQAwBAOht8H8ACCpKgZfhoT4dYX4dwRsFeoIYUb8P8ICGUQahUXqtIToTwe8qEepu8tERwVUf4Q/vzHFGeIIVNOhP0VYX/vEcMQLOeD0fKiMRlFMZUTMZ0RsLRuMaaKse0XERsbZose2n5i0WgTEb4UMRsAAs%2BIIVekMmcQMesbIXcg8tgFPOMTPlavlOcI9G%2BHvvfpdiAgiL1JygeM2ENBVD1EdrNPNItA8dMR0c8fco8iEEcdpC8SibyBCO5OVB1usHKCNF1LLOsJ9CYKoBfq0ZYWsR0XkesDeA5nGsAAmgXMAMmhADMOsBqGcNgKmjmtao4vQNpPKpROEHQgKlBryIFJkdkRsIcm/IFCNCwGAJAGuN4HCD8L1CFLlAAI5vAkAkjhrEDrAlyyQjLoTjK7TeC%2BDrB/oQCqmvC4hWkckimlGhyOQb5vzSk5FymOKBQFEumSixqVjMlJpDQVwelaqb5rTaG0kiRBDsn8mBSCnrCynUH0lBnxqJqslDTsmSlOQKQdi0AMDKl2lWlS6IZtQkAo7KlSqoxWm5SYCul6wcBzC0CcDri8CeAcBaCkCoCcALTmCbD0TLDWQsQ8CkBcLdktlzDfTrgaD6CcCSCdmaC8B9kcC8AijzmTlaBzBwCwBICqiWj0BkAUAQCHlBrRDEBcA2Tml8B0CTqUCilTmkDhBBD1CtLcC8CvnMDegGThDaDiacDjmqgkapYfk9lYAsksYgIiifmkBYCVErnwVNI3ZCawU9k4jNJ/QrDjk1RtnPlujhCvDegdZIUq4sBAUtl8AZwKB3h4CYBvBhqMCUUyCCAiBiDsBSCsXyBKBqBIW6BcD6CiQgCmDmD6Cq4iiQBzDCIRqcAahOA2nhquiYBCZjgv6nCrkVYVhYCSVQDMBJJDTsIMCkA/BiAmArBjLeA8AzBzBWDiaO4QCOAjCeCnAACcfgiYkwfQ0QCQAilmrgzQIAblpAOQRlXlxQ/QgwlQ9lAg4ujQAVmQtebQMVNQdQEwqGHugwaV8VGQLlWVXQ4V0wkgtlw5nFrZ7Zy5z5a5vUi0yAc%2BkoN5koGg6wEAuAhAJAo5pwXAMwvA2505pAs585%2BFS5pAXZPZa5G5IAW5K5NlC5HA3glV41nAvVM1cwFWaeXgQAA%3D