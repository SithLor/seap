# seap
a lib,app for lots ting like marcos and stuff

url for https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
https://godbolt.org/
idea for a program

mini js eninge in rust apps 
for amp

#[macro_export]
macro_rules! called_from {
    () => {
        format!("{}/{}:{}",file!(),line!(),column!())
    };
}
#[r]("https://godbolt.org/#z:OYLghAFBqd5QCxAYwPYBMCmBRdBLAF1QCcAaPECAMzwBtMA7AQwFtMQByARg9KtQYEAysib0QXACx8BBAKoBnTAAUAHpwAMvAFYTStJg1DEArgoKkl9ZATwDKjdAGFUtEywYgATKUcAZPAZMADl3ACNMYgkATlIAB1QFQjsGFzcPb3jE5IEAoNCWCKiuWKtMGxShAiZiAjT3Tx8yioEqmoI8kPDImMtq2vqMpv6OwK7CnpKASktUE2Jkdg4AemWAagAVAE84zDWtueI1tCw1hEjMUjWSNdpUJnQ1wzXMVVY4%2BgA6AFINAEFfn9Vms/gprlQ1gAlMwENZcT4AdgArFcFCwxLQ1lQTAwWgwwTU9kwTER0bZRLRaFtAcD0cQANaYR5MMHfABsGm%2BXgAzN8kQAhQK0MZ8gAi7I0awUqDWBHOWzWAHc6JiGKhYQoEKhFWsTHE1oEaes5Xs5gQ4iSlecGMdUCw4nRAsAlYQENc4rYWHgFExbAIFJ81opMEa1hKubyBWqAProoz0MUS65HcM8vmChjCoIQIIAN0iU0THNlMsVJHpT2IcwY6FDcu9BoUChMmB%2B/2BQkwewQBHNChAq2ArpMYU%2BaBYy3HDvoxAAtK8PiRIpO7dPIvPVIviMvvc3MAplkjotzooCI%2BmY3HgAmkeL/hawlibV6GNGq4qFNG0LQFBBWNWCBAXUkgAL0wKY1lnb5uWwNYIDMPAwKuBCwIg74EX5QE1mw25MA1ABHWogKoNlJDDblRTg/8cVhFksVIqZPgUQiCAgQtuUw/4cNw2F337YDEL2aDKOY2ox0wOg2KeMEUJDDisJw%2BhYW/fjZPIyi8AhPi1gAKjg7TZzhNCYOEp4WAAsMMIU7jsIMuFrMsyjMB/ISrK4mzbO1BQHPQ8V5Pc7CID4q4VMLf5fLPdt1gASQhA4TGAoSOXRQI2IlK4sGQAxtwNWjWQ5B8kyIKUu1yg0bRNa4SQtQDQwfJ81hShgpPQ/k1k%2BDrHMBDgZloTgkV4TwOC0UhUE4aFzClQ5FjDHkeFIQDhp60gICQKc6EichKHWmcQGILgEQ5Pg6AISJ%2BwgMJNF4MJAhqLZOHmm7mGILYAHkwm0cpFvm8c2EEV7M3upbSCwMITGAJwMX7bheCwK9xGB/BtwqfNoZG15yhJJZ5sCU6%2BuB4UwmIO6XCwK6FuIPAWAe5aqAMYAFAANTwTBFVe3Yhvm/hBBEMR2CkGRBGDNRyd0Lh9EMYwYX0PAwn7SAZlQD0Umh3hUHzYhKaweW2MsZyvpSBwa0GTxxf8MYCiKPQEiSP1UlcBpreyO3OktyY9esO22gGB2MnF5ovZGV3umKPp2hNvRzHaYOJmKGZpXmRY9EVQwCHZghooYfgad6/rBvJsaODWUxzGQOFETZT5JQgXBCBuCMuCmXhFq0GZVpAIhXC2v9iAZzheBqBnlEMZyhC1RVOd4HbImCVglhLggy/hQ6q94Jk661vRueECl%2BekbfhfUYGxdIczmDQWuiGIAAJFl%2BS7Bg/hJVB2cYJkadPgQmAv/Ar9fyfP7n1QFUfAADzCgP7n0UBw8gi0DHtqABndaDRXQCASkuYWCzhYKIAgpgGD0lIIqYmcQc76DzqQIaI1C7oMwdgpgcFcE4npBBBeS8K5Vzgpfeuc0rguHtBtFMc0m4LSulMGY9JvDckRMiLwGgvBcG5F4aIAAOUiZCOCSHzsDQuvB%2BwaBEUtNuMBEAgDwGQCgEAOCzioGqWc%2BAFCiGIOgWcuYxAtlnMwNgCgODk2nsQWebBOCsPLivEacNJb0FnAoLYhRXBozXr/EgeBUHixoLQU6c5fS4NlljeJx10nrl%2BowAgeS0kZLsZgMGwBZyBGzsDMp65AjmFMHiLBeEHi%2BiYOTbevNxACwPkoEWx9I76zxEbZwvtTa%2BBrDHK24sbY5HtukKZCyXYWxDiMz2lQRgR39qMwO0d1mx0jjsyZJzDn5A2Y3CmXZkkgA0LnDgA0KEF04H4PwjMACyaxoqQjWMzVmkRi4wjYaEzhiTBHcnFmsPha5IWN2bqImY5wHg9F1hIpRnx5FcC8MojQeKkTcmUUeJECJ1GaJedoyBeiDGtxWsY9uC8vTmO2quARAT54gpCZXdGEK7mpNkL0vegt5CDKPiNE%2BAdDYQEcLs6Z6BZnu1WSkOVyrchHLmR7A2rRTnLM2dqhg3tRiXOOf7XVjszUXPGHMmYuDbmoIeeo55lC1bjRhGsT5vz/ks0VEC4Jy8eXgo3rNKFvC2UzhDQi2ly127ICoMAbufiOVBK5QG1evg%2BUpNIJK/ZTIABiOIxkcBfG%2BLyX44mcBuRJB1jznWvI4E4PNABxYFpduUcJrhCyNYb%2BERobsIluPVxEgEkAiKuCikQHQ0GyLgbIvCHW5OSrRVDqV6EHWI9RXhl2uo4IiwxMwNZJHsJIIAA")
fast portable util
https://github.com/MicrosoftDocs/win32/blob/docs/desktop-src/Debug/system-error-codes--0-499-.md?plain=1
add();
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



@SithLor ➜ /workspaces/seap (main) $ python debug_system_error.py 
  File "/workspaces/seap/debug_system_error.py", line 117
    f'pub const {error_info["error_status"]}: &str = '
    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
SyntaxError: invalid syntax. Perhaps you forgot a comma?