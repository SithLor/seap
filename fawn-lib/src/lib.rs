#[allow(non_snake_case)]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[allow(non_snake_case)]
#[macro_export]
macro_rules! NOT_IMPLEMENTED {
    () => {
        panic!(Text!{
            "Not implemented",
            "Please implement this function"
        });
    };
}
#[macro_export]
macro_rules! Text {
    ($($x:expr),*) => {
        concat!($(stringify!($x), "\n",)*)
    };
}

pub fn asm_crash() {
    unsafe {
        use std::arch::asm;
        asm!("ud2");
    }
}
#[allow(non_snake_case)]
pub fn asm_int3() {
    unsafe {
        use std::arch::asm;
        asm!("int3");
    }
}
//https://learn.microsoft.com/en-us/windows/win32/api/ntdef/ns-ntdef-list_entry
//https://learn.microsoft.com/en-us/windows/win32/api/ntdef/ns-ntdef-_unicode_string
//http://undocumented.ntinternals.net/index.html?page=UserMode%2FStructures%2FRTL_DRIVE_LETTER_CURDIR.html
//https://doxygen.reactos.org/d3/d61/include_2ndk_2pstypes_8h_source.html#l00643
//https://www.crow.rip/crows-nest/mal/dev/inject/syscalls/indirect-syscalls
//https://www.nirsoft.net/kernel_struct/vista/TEB.html
//https://klezvirus.github.io/RedTeaming/AV_Evasion/NoSysWhisper/
//https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-dtyp/262627d8-3418-4627-9218-4ffe110850b2
//https://lenarn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-
//https://www.vergiliusproject.com/kernels/x64/Windows%2010%20%7C%202016/2110%2021H2%20(November%202021%20Update)/_OBJECT_ATTRIBUTES
//https://www.vergiliusproject.com/kernels/x64/Windows%2011
//https://www.geoffchappell.com/studies/windows/km/ntoskrnl/inc/api/pebteb/peb/index.htm
//http://undocumented.ntinternals.net/index.html?page=UserMode%2FStructures%2FRTL_DRIVE_LETTER_CURDIR.html
//https://www.virustotal.com/gui/file/ef1d16f11b184f9017bef21db833313c427f1e10dd0adf37b879c2e2f71548d5/behavior
//https://godbolt.org/#z:OYLghAFBqd5QCxAYwPYBMCmBRdBLAF1QCcAaPECAMzwBtMA7AQwFtMQByARg9KtQYEAysib0QXACx8BBAKoBnTAAUAHpwAMvAFYTStJg1DEArgoKkl9ZATwDKjdAGFUtEywZ7HAGTwNMAHLuAEaYxBIA7ABspAAOqAqEdgwubh568Ym2Ar7%2BQSyh4VzRlpjW2QxCBEzEBKnunlyl5clVNQS5gSFhkTHm7fXpTf21nfmFvQCUlqgmxMjsHAD0SwDUACoAnrGYq5uzxKtoWKsIYZikqySrtKhM6KuGq5iqrLH0AHQApBoAgj%2B/FarX4KK5UVYAJTMBFWXA%2BEQArJcFCwxLRVlQTAwbMlQTVdkwTERUbZRLRaJsAUDUcQANaYB5MUFfKIaL4AJgAzF8EQAhPy0PyYHkAERZGlWClQqwIZ02qwA7nR0QxUDCFAhUArViZYqs/FS1rLdrMCLEiYqzgwjqgWLE6H5gIrCAgrrFbCw8AomBUFB9VophX8geKOdy%2BaqAPqooz0UXiq6HUNcnn8hiC/wQfwANzCk3jrJl0oVJFpj2IswY6ENMoQXv1CgUJkw32DayEmF2CAIZoUIBWwBdJmCHzQLCWY/t9GIAFoXu8SGEJ7ap2E56oF8Ql16m5gFEsEQBOTmHgEA1YXnVKSUEdAgEA1ZBIB8or6c3nny9mXbmO8gBL3sQTAKveyCRtmqB4NW75nn8l6rCs5rBKsLAYJKmzmJgLCRmEFbEJGZyqG%2BH5wZeiHDihaEKBhBBYThxB4ZGDDuMRn4XuRyGoQ81GYdhuEkAR7iGKxfxsQhSzdr2/ZLPQNQMB8nrIBWUpUAQo62ksjAzmYSxKlWWr7npnLsksTD2ksghYFQFkKDOlmYFQM6CuYOGCMQlKkexEk9rEfYrLJxDyYpymoKp6njlpOl6egBm6X4xmmeZ9nWQwtnJTOkZYngxyYJG5jEI6YkrJJsTSViMXIO4jC0egHyCH4tGBWIfr%2BAQSx%2BFgqgfN2LC0G%2BABisRMMAwqciKgbEAAshgwrsgi7L9VUpg2HMe4cvN/UQus3iRiKEIAJIAGrYJG3jYOs6zYBCkZOHIEIivtELdQQvVFd5UkrDFqibCN8lbkwNgJB8JDAEs6CcmDURcO12JuFgkbslWtII75BDbHukYABwIHlBwLM9r1crQGgaFEkjcqJnmrGGqZbrExAQE4%2BYImKVNITey0wnIAT7U4ADyIonUI6wHQEADi1MRCRvzwfB3iMMAsogAGQgABJ81tpBifBk1MKoeAsO48tGErKvq5r2uXryJhUFQPRmxr6yRsoIuXECygAOrCxCDaPKs8QNWERZ%2B0qJz5Y6lwKnWT6%2B3TW5KIIDKPKCABUhswiYXBRPq1pQuYYlfBErMy5eNN8nHjPM8X8Hs/lJg2Ks3j7cLkbYAEIsAJqS9LsuXv1Ga0sracWk3Ldt53WtU/BvID0P6eN83TvjxCHeTyXF6F9XpcpuXmD05X8ZsxRdcN1tO17UdJ1nRdV03XdD0%2B4XPe96s/fDX2DsW1Pl7G4rSCf%2BsNeL8NgG0wG0O0ys5DeD5uLIBL8RQJGUD6f%2B3NeYCyFiLfasCC5FwLjvXkFcmaH3Xv7Y%2BBBOaQm2pGOQQhb7KAhHzJw2AhBCGdr8CEvxJrnSukIbulsLy631obFgv9TZQJgWLOBvdREoOgdg7%2BF437AA/uI%2BRJD4IikwMEEwwAlEqLkZI/hqwXCpVcJgVWhh0DiFWMoQ6fN9oiikbLExUp6B6MgQYpx8EqjoH2gwc0BALFVmsarX4AQRRnS8ZeHxfMiQBKCVY9gqxQnhMiUYnx2AGIkASSEsJETsBRIvE4OYW5BAijwFuQG7kkFiJ5vzQWkZvZYMMQo4xJTqrlMqUQdyOSkkpPyYU1YIpyQ1JQXU9BjTMFqOAftVEI1RkBFYEk1B9SMGixaeoy8LgWAxnQJ0SB4yGlNOmS/bADBswVIEGwQQytbH2Mcek6otRHTKASEkHIDkCAeIkYMtozyjCvKyMkdYqBSoBk8UYj2UExEQtaarTAeBgDdm%2BSc3uTgEA1ChegGFPyjHopqPCxFyLwW4taS4sx6wXgEF%2BD2Aq2jaL6NJZsi8UL9IKncSS1FsshCagVKymK7KDDKJRRs4B/KtTrEINYlZEzjmivgXuWkRBYiLLYActBRypnyt7jysotA/H8HVasyZ6zBlQnqmwEUPomBGtlVqwZETikMQ6RUzAVTNjK1TGfXaB1jqnXOpda6t17qPWIqsDQqh2RshZk4zeeDwwEL3gzIhLMxK13IfXGEyhsC8lOntXavx1gyyfkYmRIrBl%2BKSGIPAAAvBkyteR8z5mdMJvyFC9NuXYhxFaGDeDuOgPmxAsBTQwCYegvhzDK1HkvduK8e2TSwiQTYg7h3TXQGOzAE6vkLzHrO1eRi/GVtsNWmtPpkgrrCGujdW6p2L1bnu2NuCqZl0TfvFNW8Lzpoodm3Nm1sAnV5NApwABpPhrSAhUrnhaH9kY/0AaA8B35tblmwpIXG59%2BDCFVzTWQ79OawPMtWH4s4BUaq/HQOgeOCghBDQWMrYIqBXBmswPcWZw1MD9ToDgVQbq%2BbulxPRxjtBBm8gRUYTR2jgAjTvKsBjTH0lDS3IJ%2BTrTJpEkMNuu53aD1zMwLyJkmByOUb3B/LTDzWl9vuGEK11QoNZpzXm66IpC2/Gdq7IxygKwLEbEgoCbBGof29TQuhDCmEsLYRwrhgbWEuwhL84cQgaJYRszamxXbzOEc86gbz7aWNgrM4M/qTICDKC0X25Ag80v3MK8V0rwRyu0ghKaIUdmbE5oQwwuQ6wsEnVizV8wdW5Dpmy415r/hWs/o63zLrPW3NxaMWci5FYGDXPkLEdAPpMAuCxNu1R2rZbAbCP4WgTg0TBABrSdYTBgjWLM3NwZ2BcyCD7cADsOJ7BVe060x71UXuduq0Y/qW5N1em3cPezv6IT/sjIBxhwH7tGPWLQBQ2ANyGESAIbbidwicv2/BJHCh%2BQvTMv9r7hGCdE9RLEInH9Ux7bDeyUUzH7h83TJsHl%2BJ0ALtQu5fTShScZeARCFjA62cc63FzxdPS8sC%2BZ6LikbRSQdmILmYgKXZdGN%2BKlPALgsBIJGurz7guX58ywrrlQ7HDcFaMUN7KM1TtKCuzdzAVv0uDPGGEPmVAss5ZIIyrl8EAgEDFrcc7tAlHloU/idknq%2BQmExmGyQTO8WkeymIN7FRJVsFNMrPAZNBnwrMh2YAq3hdKBVyhplwDC%2BxGL6t7Znpduoer3lzRDfCAguqOH4H6wEDx01LQGTe2C%2Bt627aRvQPOwzxG73/vrgh/N5fh74gXua/%2B7x5eQRBt3DL9X3l9fgyfcmbXwLhHrSxb4HFwyXpTvbtu4815kzfzGrwtoDsHH1vz/4BFE4Gl5C8D0og6TpG6DKWbDoNYa6tJ8xCC6zaAkCHRhAY6eC46DLQGTR%2BDwGIHJCR5QFCDWx0DoDL44GEbQHKAGAED8DEAsD7QL5V4vxsYjRCAJZJYsDEEzK6ZMHBCJa8SwGYHEBIFsH0EcHME8EYHEAIH8HYEoFGIX54C9LWy2z2x04GJhqchJ4xoeYJAlaP6NhHpNZEgtbSGtIE6o5DTa4CCU4k5GHk7I6mHo7JCWHU6EC05x4J7viwjsgJ4aGtIdiNjJA0GCEbxPr/BUwrBza%2BwBzY7BzGgcyZo4bIRow7Bta5qxbUxjSrDg7JEiQkLsyJG7DeoXx%2BrXyBp3whrXSpFvgigZHzwFG%2BpXwBq3zBoPzZE1wUR5GUI7TBblGhbMIxbsKcLcIQgxYixpFVGZFBa0LdGMK9ERYDHRYtGXi5How2IyqarrKjHVEWirFrLNILGfptHLHTr3qdzhGVGbEwhHHLwdx7GkIJHLEQiXF7qnHpGZGPGdw3FLFJEwbeD5rOZFrPFjHzzfG/EuYfEHFfEOZwYw4IYAnnHJGwZQ7wZw5gl3EQmw4gadbdYBC9YjFnFYjeh2wYgMAQCZFgQQRQT5gwRHyom7BmYbGkngSQTQTPz7E0lG6wmZFmYokyjLE0LmzrAbGZxRDcntF8mOwcnzxilbQim8kGKCnGQylJF7YSlbEqFUk5Hgm7D9JnQbFcnqmtFsmNrNrYBhKCluG8jiSrC8gdyXS%2ByJFp7kjyhxx7jVRJxMg6iYw5yQjQjxE8lJFGktoBAqkwgBkmkBCKm7AigewaxVF4kKnqkvpRgxjABxipp/DsxUDWj7SE5ibAASY6LSYQCTCrAzhvjYCyZCYEbwT0Awjzw7DBC3IOaxY3H4lMCEklqtJMgsBgCQAcjsioTZirBASqCXAACOJYQ6/s5CqwwqqYEarI8Y7I7IlwpoEAfZw5fZxZ9ZlJLJksH61MVMrZ7ZUsqwJJ25Hwomjo%2BZUmSc6G/wwRNY%2B04I%2BwJgV4uw4oqIfgRZ4olwWAyABgW4%2BoMI7p4oSECYRAkonYQFXpMRpoASIANYGZ1on5xJxZT8qwHwmFe5AIHA0wtAnACIvAngHAWgpAqAnAec6oeM75XIPApAXyJFuF0wg8XAmMHwnIXAXACIiInhnhCIkgUQMQ%2BFHAkgRFmgvA5FHAvAfYGg9F4l0wcAsASAk4XGZAFAEAKl04IAxAxQrIfAdAAWlADZjFpAwQfgNQHq3AvAZlzA7kfMwQ2gbqDFdFY4q2rOFI4lpAWAkmp25IfYVlXlWEhgKZiwpF%2BAXSeAuY/lpFLwbqRIiwdFgcwlpFgowQQE7k5unl/%2BrBVl0wVAQqCgh0eAmACofGjAnAdF/AggIgYg7AUgMggggYagnlugTQBgRg2l0I%2BgABfYkA0woKvonAM4TgboBATkmAuYfUY0NaJZI1qAqu%2BVWoM4T4bqtICglRVAzUuww1HMeAsQlR3lOifg/AElquBUWAvVUAzAaqj4CApA2YYgzYIAqgmMUQkYZMM4sQyAM40UBkM4wAzEkw0wVgTl2BEAjggwjQpAPgQoRBTQmQ7yKQrgDQGQbyFQYw3QRQzQoNAgz%2BkNegIN72lQTyHQsNmNBNJN%2BNwwJNGNBQPQXAwN1FEgeFBFYlJlklQ50IyAsI8IUQHwEoEAuAhA1wYYDNvADFWgClylK4ql5AlAmlPQOl0QslNAtAhlEAxlpFNlFlFV1l5ldlDlTlutpArl1U7lllYVWiOivlyOxtWAyZ4gJl4VoNUVnlsVlUtExtSVnlqV6VmwmVJl2VuteVBVRVJVZVxFlVsgNV4g9VVV8gSgzVJlugy57VxgXVqVl1/V/GAg/lO1A141k1lRnIs1VwC1twCoy1ZwFW61Y0m1yO21I14c%2B1Y0h1g4DAJ1ZFZ1UEe48AwNZQONng4NVYVN0NVYtNEw8NaNyQo9CN6NZNdNWNhNFQeNyNQw2NRNz%2BE99NlglNa9UNIwpNeQ5NDNMwcwdGp9wlhFpAxFpFHNpg5g3NcI0Q/Np5Qt3S1MtFkw4t8lpAEASA%2BAtsct1A0dZIdV0g8dTV6gJltACAfYbVcDgDqk6MIAslsDSgqg5CTAA12BUlpAxAcDIAy5BDCgSDeRqD%2BDcDVKQEOD9gUlLNHA19t9ElnA5StsqwYdCoQcD9BAT9vNr92YoIPDfDL9pFEtTFpAg8kgbFnICICIGgh4XAnImMyjXAUaqdnAolN9nlkl0leg4jClMAiAKAVAwAwDCtxAqqiwwjPNojvADIwt51egKdG9NU/UWIRNnA2Zl54mVtN5d44twOUEqDDDTDOjnATg/UEsNjz9fNAt79IttFlw2yq4SYXI7I39cljFQNkjIAnInI7F%2BTRTxTxT%2BgmjbNd9nAejslBjDD7IFTLDeDtT91WB9gkgQAA%3D


pub mod ms;
pub mod custom;
pub mod helper;
//pub mod error;
pub mod dectect;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
