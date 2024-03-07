use windows::{ Win32::Foundation::*, Win32::System::SystemServices::*, };
use windows::{ core::*, Win32::UI::WindowsAndMessaging::MessageBoxA, };

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: u32,
    _: *mut ())
    -> bool
{
    match call_reason {
        DLL_PROCESS_ATTACH => process_attach(),
        DLL_PROCESS_DETACH => process_detach(),
        DLL_THREAD_ATTACH => thread_attach(),
        DLL_THREAD_DETACH => thread_detach(),
        _ => ()
    }

    true
}

fn process_attach() {
    unsafe {
        // Create a message box
        MessageBoxA(HWND(0),
            s!("ZOMG!"),
            s!("hello.dll"),
            Default::default()
        );
    };
}
fn process_detach() {
    unsafe {
        // Create a message box
        MessageBoxA(HWND(0),
            s!("GOODBYE!"),
            s!("hello.dll"),
            Default::default()
        );
    };
}
fn thread_detach(){
    unsafe {
        // Create a message box
        MessageBoxA(HWND(0),
            s!("GOODBYE!"),
            s!("hello.dll"),
            Default::default()
        );
    };
}
fn thread_attach(){
    unsafe {
        // Create a message box
        MessageBoxA(HWND(0),
            s!("ZOMG!"),
            s!("hello.dll"),
            Default::default()
        );
    };
}
