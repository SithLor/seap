/// LINK:https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-exitwindows
pub fn exit_windows() -> bool {
    use windows::Win32::System::Shutdown::ExitWindowsEx;
    use windows::Win32::System::Shutdown::EWX_FORCE;
    use windows::Win32::System::Shutdown::SHUTDOWN_REASON;
    let result:Result<(), windows::core::Error> = unsafe { ExitWindowsEx(EWX_FORCE, SHUTDOWN_REASON(0)) };
    result.is_ok()
}

pub fn lock_work_station() -> bool {
    use windows::Win32::System::Shutdown::LockWorkStation;
    let result: Result<(), windows::core::Error>= unsafe { LockWorkStation() };
    result.is_ok()
}
pub fn initiate_system_shutdown_a() -> bool {
    use windows::Win32::System::Shutdown::InitiateSystemShutdownA;
    let result: Result<(), windows::core::Error> = unsafe { InitiateSystemShutdownA(None, None, 0, true, true) };
    result.is_ok()
}
pub fn Beep(){}

