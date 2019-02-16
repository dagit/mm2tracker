#![windows_subsystem = "windows"]

use std::io::Error;
use winapi::shared::windef::HWND;

static APP_NAME : &str = "mm2tracker";

fn as_wstr(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

struct Window {
    handle : HWND,
}

fn create_window(name: &str, title: &str) -> Result<Window, Error> {
    use std::ptr::null_mut;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::um::winuser::{
        DefWindowProcW,
        RegisterClassW,
        CreateWindowExW,
        WNDCLASSW,
        CS_OWNDC,
        CS_HREDRAW,
        CS_VREDRAW,
        CW_USEDEFAULT,
        WS_OVERLAPPEDWINDOW,
        WS_VISIBLE,
    };

    let name = as_wstr(name);
    let title = as_wstr(title);

    let hinstance = unsafe { GetModuleHandleW( null_mut() ) };

    let wnd_class = WNDCLASSW {
        style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc : Some( DefWindowProcW ),
        hInstance: hinstance,
        lpszClassName: name.as_ptr(),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hIcon: null_mut(),
        hCursor: null_mut(),
        hbrBackground: null_mut(),
        lpszMenuName: null_mut(),
    };
    unsafe { RegisterClassW( &wnd_class ) };

    let handle = unsafe { CreateWindowExW(
        0,
        name.as_ptr(),
        title.as_ptr(),
        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        null_mut(),
        null_mut(),
        hinstance,
        null_mut(),
    ) };

    if handle.is_null() {
        Err( Error::last_os_error() )
    } else {
        Ok( Window { handle } )
    }
}

fn handle_message(window: &mut Window) -> bool {
    use std::mem;
    use winapi::um::winuser::{
        TranslateMessage,
        DispatchMessageW,
        GetMessageW,
        MSG
    };

    unsafe {
        let mut message : MSG = mem::uninitialized();


        if GetMessageW(&mut message as *mut MSG, window.handle, 0, 0) > 0 {
            TranslateMessage(&message as *const MSG);
            DispatchMessageW(&message as *const MSG);
            true
        } else {
            false
        }
    }
}

fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let ret = unsafe {
        MessageBoxW(null_mut(),
                    as_wstr(msg).as_ptr(),
                    as_wstr(APP_NAME).as_ptr(),
                    MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}

fn main() {
    let mut window = create_window(APP_NAME,APP_NAME).expect("Failed to create window");

    loop {
        if !handle_message( &mut window ) {
            break;
        }
    }
}
