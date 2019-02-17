#![windows_subsystem = "windows"]

use std::io::Error;
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::{
    UINT,
    WPARAM,
    LPARAM,
    LRESULT,
};
use winapi::um::winnt::HANDLE;

static APP_NAME : &str = "mm2tracker";

// TODO: It should be possible to extract these dimensions
// from the loaded images
const ROBO_PORTRAIT_WIDTH  : i32 = 54;
const ROBO_PORTRAIT_HEIGHT : i32 = 60;
const ITEM_PORTRAIT_WIDTH  : i32 = 37;
const ITEM_PORTRAIT_HEIGHT : i32 = 20;

enum ContextMenu {
    Reset = 1,
    Exit = 2,
}

const ROBO_PORTRAIT_FILENAMES : [&str; 8] = [
    "../assets/bubbleman-60.bmp",
    "../assets/airman-60.bmp",
    "../assets/quickman-60.bmp",
    "../assets/heatman-60.bmp",
    "../assets/woodman-60.bmp",
    "../assets/metalman-60.bmp",
    "../assets/flashman-60.bmp",
    "../assets/crashman-60.bmp"
];

const ITEM_PORTRAIT_FILENAMES : [&str; 3] = [
    "../assets/item1-20.bmp",
    "../assets/item2-20.bmp",
    "../assets/item3-20.bmp",
];

fn as_wstr(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

struct Window {
    handle : HWND,
}

fn load_bitmap(filename: &str) -> Result<HANDLE, Error> {
    use std::ptr::null_mut;
    use winapi::um::winuser::{
        LoadImageW,
        IMAGE_BITMAP,
        LR_DEFAULTSIZE,
        LR_LOADFROMFILE,
    };
    let handle = unsafe { LoadImageW(
        null_mut(),
        as_wstr(filename).as_ptr(),
        IMAGE_BITMAP,
        0, 0, LR_DEFAULTSIZE | LR_LOADFROMFILE)
    };

    if handle.is_null() { return Err( Error::last_os_error() ) };
    Ok(handle)
}

fn create_window(name: &str, title: &str) -> Result<Window, Error> {
    use std::ptr::null_mut;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::shared::windef::{
        RECT,
    };
    use winapi::um::winuser::{
        AdjustWindowRectEx,
        CreateWindowExW,
        DefWindowProcW,
        GetWindowLongW,
        LoadCursorW,
        MoveWindow,
        RegisterClassW,
        SendMessageW,
        WNDCLASSW,
        BM_SETIMAGE,
        BS_BITMAP,
        BS_AUTOCHECKBOX,
        BS_PUSHLIKE,
        CS_OWNDC,
        CS_HREDRAW,
        CS_VREDRAW,
        CW_USEDEFAULT,
        GWL_STYLE,
        GWL_EXSTYLE,
        IDC_ARROW,
        IMAGE_BITMAP,
        WS_OVERLAPPEDWINDOW,
        WS_VISIBLE,
        WS_CHILD,
    };

    let name = as_wstr(name);
    let title = as_wstr(title);

    let hinstance = unsafe { GetModuleHandleW( null_mut() ) };

    let wnd_class = unsafe { WNDCLASSW {
        style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc : Some( window_proc ),
        hInstance: hinstance,
        lpszClassName: name.as_ptr(),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hIcon: null_mut(),
        hCursor: LoadCursorW(null_mut(), IDC_ARROW),
        hbrBackground: null_mut(),
        lpszMenuName: null_mut(),
    }};
    unsafe { RegisterClassW( &wnd_class ) };

    let handle = unsafe { CreateWindowExW(
        0,
        name.as_ptr(),
        title.as_ptr(),
        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        0,
        0,
        null_mut(),
        null_mut(),
        hinstance,
        null_mut(),
    ) };

    if handle.is_null() { return Err( Error::last_os_error() ) }

    //calculate the window size based on a desired client rect size
    let robo_count = ROBO_PORTRAIT_FILENAMES.len() as i32;
    let item_count = ITEM_PORTRAIT_FILENAMES.len() as i32;

    let mut window_rect = RECT {
        left: 0,
        top: 0,
        right: ROBO_PORTRAIT_WIDTH * robo_count + ITEM_PORTRAIT_WIDTH,
        bottom: std::cmp::max(ROBO_PORTRAIT_HEIGHT, ITEM_PORTRAIT_HEIGHT * item_count),
    };
    let ok = unsafe { AdjustWindowRectEx(
        &mut window_rect,
        GetWindowLongW(handle, GWL_STYLE) as u32,
        false as i32,
        GetWindowLongW(handle, GWL_EXSTYLE) as u32 )
    };
    if ok == 0 { return Err( Error::last_os_error() ) }

    //Now resize the window
    let ok = unsafe { MoveWindow(
        handle,
        0, 0,
        window_rect.right - window_rect.left,
        window_rect.bottom - window_rect.top,
        true as i32)
    };
    if ok == 0 { return Err( Error::last_os_error() ) }

    let button_style = BS_BITMAP | WS_VISIBLE | WS_CHILD | BS_AUTOCHECKBOX | BS_PUSHLIKE;

    let robo_images : Vec<HANDLE> = ROBO_PORTRAIT_FILENAMES
        .iter()
        .map(|n| load_bitmap(n).expect("Failed to load asset"))
        .collect();
    let item_images : Vec<HANDLE> = ITEM_PORTRAIT_FILENAMES
        .iter()
        .map(|n| load_bitmap(n).expect("Failed to load asset"))
        .collect();

    // Place the buttons for each robo master
    for i in 0..robo_images.len() {
        let _hbtn : HWND = unsafe { CreateWindowExW(
            0,
            as_wstr("BUTTON").as_ptr(),
            as_wstr("").as_ptr(),
            button_style,
            i as i32*ROBO_PORTRAIT_WIDTH, 0, ROBO_PORTRAIT_WIDTH, ROBO_PORTRAIT_HEIGHT,
            handle,
            null_mut(),
            null_mut(),
            null_mut() )
        };
        unsafe {
            SendMessageW (
                _hbtn,
                BM_SETIMAGE,
                IMAGE_BITMAP as usize,
                robo_images[i] as isize,
            );
        }
    }

    // Place the buttons for each item
    for i in 0..item_images.len() {
        let _hbtn : HWND = unsafe { CreateWindowExW(
            0,
            as_wstr("BUTTON").as_ptr(),
            as_wstr("").as_ptr(),
            button_style,
            robo_count*ROBO_PORTRAIT_WIDTH, i as i32*ITEM_PORTRAIT_HEIGHT, ITEM_PORTRAIT_WIDTH, ITEM_PORTRAIT_HEIGHT,
            handle,
            null_mut(),
            null_mut(),
            null_mut() )
        };
        unsafe {
            SendMessageW (
                _hbtn,
                BM_SETIMAGE,
                IMAGE_BITMAP as usize,
                item_images[i] as isize,
            );
        }
    }

    Ok( Window { handle } )

}

pub unsafe extern "system" fn window_proc(hwindow: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT
{
    use std::ptr::null_mut;
    use winapi::um::winuser::{
        DefWindowProcW,
        PostQuitMessage,
        CreatePopupMenu,
        InsertMenuW,
        TrackPopupMenu,
        MF_BYPOSITION,
        MF_STRING,
        MF_ENABLED,
        MF_SEPARATOR,
        WM_DESTROY,
        WM_CONTEXTMENU,
        WM_NOTIFY,
        LPNMHDR,
        TPM_TOPALIGN,
        TPM_LEFTALIGN,
        TPM_RETURNCMD,
    };
    use winapi::um::commctrl::{
        NM_CUSTOMDRAW,
    };
    use winapi::shared::minwindef::BOOL; // this is really c_int
    use winapi::shared::windowsx::{
        GET_X_LPARAM,
        GET_Y_LPARAM,
    };

    if msg == WM_DESTROY {
        PostQuitMessage(0);
    } else if msg == WM_NOTIFY {
        let pnm = lparam as LPNMHDR;
        if (*pnm).code == NM_CUSTOMDRAW {
            //custom_draw_button((*pnm).hwndFrom, lparam as LPNMCUSTOMDRAW);
        }
    } else if msg == WM_CONTEXTMENU {
        let menu = CreatePopupMenu();
        InsertMenuW(menu, -1i32 as u32, MF_BYPOSITION | MF_STRING | MF_ENABLED, ContextMenu::Reset as usize, as_wstr("Reset").as_ptr());
        InsertMenuW(menu, -1i32 as u32, MF_BYPOSITION | MF_SEPARATOR, 0, null_mut());
        InsertMenuW(menu, -1i32 as u32, MF_BYPOSITION | MF_STRING | MF_ENABLED, ContextMenu::Exit as usize, as_wstr("Exit").as_ptr());
        let selection = TrackPopupMenu(menu, TPM_RETURNCMD | TPM_TOPALIGN | TPM_LEFTALIGN, GET_X_LPARAM(lparam), GET_Y_LPARAM(lparam), 0, hwindow, null_mut());
        if selection == ContextMenu::Exit as BOOL {
            PostQuitMessage(0);
        } else if selection == ContextMenu::Reset as BOOL {
            // TODO
        }
    }

    DefWindowProcW(hwindow, msg, wparam, lparam)
}

//fn custom_draw_button(hwnd: HWND, nmc: &NMCUSTOMDRAW) -> LRESULT
//{
//}

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

