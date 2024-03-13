
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::LibraryLoader::GetProcAddress;
use windows::Win32::Foundation::HMODULE;
use windows::core::Error;
use windows::core::w;
use windows::core::s;

pub fn isWine() -> bool {
    let mode: i32 = 0;
    if mode == 1 {
        return std::env::var("WINELOADERNOEXEC").is_ok();
    } else {
        let e: Result<HMODULE, Error> = unsafe {
            GetModuleHandleW(w!("kernel32.dll"))
        }; 
        let h: HMODULE = e.unwrap();
        let wine_get_unix_file_name = unsafe {
            GetProcAddress(h, s!("wine_get_unix_file_name"))
        };
        return wine_get_unix_file_name.is_some();
    }
}