#![windows_subsystem = "windows"]

use std::io::Error;
use winapi::shared::windef::{
    HWND,
    HBITMAP,
};
use winapi::shared::minwindef::{
    UINT,
    WPARAM,
    LPARAM,
    LRESULT,
};
use winapi::um::commctrl::{
    LPNMCUSTOMDRAW,
};

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

const ROBO_PORTRAIT_NAMES : [(&str, &str); 8] = [
    ("IMG_BUBBLEMAN", "IMG_BUBBLEMAN_X"),
    ("IMG_AIRMAN",    "IMG_AIRMAN_X"),
    ("IMG_QUICKMAN",  "IMG_QUICKMAN_X"),
    ("IMG_HEATMAN",   "IMG_HEATMAN_X"),
    ("IMG_WOODMAN",   "IMG_WOODMAN_X"),
    ("IMG_METALMAN",  "IMG_METALMAN_X"),
    ("IMG_FLASHMAN",  "IMG_FLASHMAN_X"),
    ("IMG_CRASHMAN",  "IMG_CRASHMAN_X"),
];

const ITEM_PORTRAIT_NAMES : [(&str, &str); 3] = [
    ("IMG_ITEM1_BW", "IMG_ITEM1"),
    ("IMG_ITEM2_BW", "IMG_ITEM2"),
    ("IMG_ITEM3_BW", "IMG_ITEM3"),
];

fn as_wstr(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

struct Window {
    handle: HWND,
    robo_buttons: Vec<HWND>,
    robo_images:  Vec<(HBITMAP,HBITMAP)>,
    item_buttons: Vec<HWND>,
    item_images:  Vec<(HBITMAP,HBITMAP)>,
}

impl Window {
    fn new() -> Self {
        use std::ptr::null_mut;
        Window {
            handle: null_mut(),
            robo_buttons: vec![],
            robo_images:  vec![],
            item_buttons: vec![],
            item_images:  vec![],
        }
    }
}

fn initialize_window(window: &mut Window, name: &str, title: &str) -> Result<(), Error> {
    use std::ptr::null_mut;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::um::winuser::{
        CreateWindowExW,
        LoadCursorW,
        RegisterClassW,
        WNDCLASSW,
        CS_OWNDC,
        CS_HREDRAW,
        CS_VREDRAW,
        CW_USEDEFAULT,
        IDC_ARROW,
        WS_VISIBLE,
    };
    use winapi::shared::minwindef::LPVOID;

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
        WS_VISIBLE,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        0,
        0,
        null_mut(),
        null_mut(),
        hinstance,
        window as *mut Window as LPVOID,
    ) };

    if handle.is_null() { return Err( Error::last_os_error() ) }
    window.handle = handle;
    Ok( () )
}

fn layout_window(window: &mut Window) -> Result<(), Error> {
    use std::ptr::null_mut;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::shared::minwindef::{
        HINSTANCE,
    };
    use winapi::shared::windef::{
        RECT,
    };
    use winapi::um::winuser::{
        AdjustWindowRectEx,
        CreateWindowExW,
        GetWindowLongW,
        MoveWindow,
        BS_BITMAP,
        BS_AUTOCHECKBOX,
        BS_PUSHLIKE,
        GWL_STYLE,
        GWL_EXSTYLE,
        WS_VISIBLE,
        WS_CHILD,
    };

    //calculate the window size based on a desired client rect size
    let robo_count = ROBO_PORTRAIT_NAMES.len() as i32;
    let item_count = ITEM_PORTRAIT_NAMES.len() as i32;

    let mut window_rect = RECT {
        left: 0,
        top: 0,
        right: ROBO_PORTRAIT_WIDTH * robo_count + ITEM_PORTRAIT_WIDTH,
        bottom: std::cmp::max(ROBO_PORTRAIT_HEIGHT, ITEM_PORTRAIT_HEIGHT * item_count),
    };
    let ok = unsafe { AdjustWindowRectEx(
        &mut window_rect,
        GetWindowLongW(window.handle, GWL_STYLE) as u32,
        false as i32,
        GetWindowLongW(window.handle, GWL_EXSTYLE) as u32 )
    };
    if ok == 0 { return Err( Error::last_os_error() ) }

    //Now resize the window
    let ok = unsafe { MoveWindow(
        window.handle,
        0, 0,
        window_rect.right - window_rect.left,
        window_rect.bottom - window_rect.top,
        true as i32)
    };
    if ok == 0 { return Err( Error::last_os_error() ) }

    let button_style = BS_BITMAP | WS_VISIBLE | WS_CHILD | BS_AUTOCHECKBOX | BS_PUSHLIKE;
    let hinstance = unsafe { GetModuleHandleW( null_mut() ) };
    fn load(hinst: HINSTANCE, (unchecked, checked): &(&str, &str)) -> (HBITMAP, HBITMAP)
    {
        use winapi::um::winuser::LoadBitmapW;
        unsafe {
            (LoadBitmapW(hinst, as_wstr(unchecked).as_ptr()),
             LoadBitmapW(hinst, as_wstr(checked).as_ptr()))
        }
    }
    let robo_images : Vec<(HBITMAP, HBITMAP)> = ROBO_PORTRAIT_NAMES
        .iter()
        .map(|n| load(hinstance, n))
        .collect();
    let item_images : Vec<(HBITMAP, HBITMAP)> = ITEM_PORTRAIT_NAMES
        .iter()
        .map(|n| load(hinstance, n))
        .collect();

    // Place the buttons for each robo master
    for i in 0..robo_images.len() {
        let hbtn : HWND = unsafe { CreateWindowExW(
            0,
            as_wstr("BUTTON").as_ptr(),
            as_wstr("").as_ptr(),
            button_style,
            i as i32*ROBO_PORTRAIT_WIDTH, 0, ROBO_PORTRAIT_WIDTH, ROBO_PORTRAIT_HEIGHT,
            window.handle,
            null_mut(),
            null_mut(),
            null_mut() )
        };
        window.robo_buttons.push(hbtn);
    }

    // Place the buttons for each item
    for i in 0..item_images.len() {
        let hbtn : HWND = unsafe { CreateWindowExW(
            0,
            as_wstr("BUTTON").as_ptr(),
            as_wstr("").as_ptr(),
            button_style,
            robo_count*ROBO_PORTRAIT_WIDTH, i as i32*ITEM_PORTRAIT_HEIGHT, ITEM_PORTRAIT_WIDTH, ITEM_PORTRAIT_HEIGHT,
            window.handle,
            null_mut(),
            null_mut(),
            null_mut() )
        };
        window.item_buttons.push(hbtn);
    }
    window.robo_images = robo_images;
    window.item_images = item_images;

    Ok(())
}

pub unsafe extern "system" fn window_proc(hwindow: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT
{
    use std::ptr::null_mut;
    use winapi::um::winuser;
    use winapi::um::winuser::{
        DefWindowProcW,
        PostQuitMessage,
        CreatePopupMenu,
        InsertMenuW,
        TrackPopupMenu,
        GetWindowLongPtrW,
        SetWindowLongPtrW,
        SetWindowPos,
        CREATESTRUCTW,
        MF_BYPOSITION,
        MF_STRING,
        MF_ENABLED,
        MF_SEPARATOR,
        WM_DESTROY,
        WM_CONTEXTMENU,
        WM_NOTIFY,
        WM_NCCREATE,
        LPNMHDR,
        GWLP_USERDATA,
        TPM_TOPALIGN,
        TPM_LEFTALIGN,
        TPM_RETURNCMD,
        SWP_NOMOVE,
        SWP_NOSIZE,
        SWP_NOZORDER,
        BM_SETCHECK,
        BST_UNCHECKED,
    };
    use winapi::um::commctrl::{
        NM_CUSTOMDRAW,
        LPNMCUSTOMDRAW,
    };
    use winapi::shared::minwindef::BOOL; // this is really c_int
    use winapi::shared::windowsx::{
        GET_X_LPARAM,
        GET_Y_LPARAM,
    };

    if msg == WM_NCCREATE {
        SetWindowLongPtrW(hwindow, GWLP_USERDATA, (*(lparam as *mut CREATESTRUCTW)).lpCreateParams as isize);
        SetWindowPos(hwindow, null_mut(), 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER);
    } else if msg == WM_DESTROY {
        PostQuitMessage(0);
    } else if msg == WM_NOTIFY {
        let pnm = lparam as LPNMHDR;
        let window = std::mem::transmute::<isize, &mut Window>(GetWindowLongPtrW(hwindow, GWLP_USERDATA));
        if window.handle.is_null() { PostQuitMessage(0); }

        if (*pnm).code == NM_CUSTOMDRAW &&
            (window.robo_buttons.contains(&(*pnm).hwndFrom) ||
             window.item_buttons.contains(&(*pnm).hwndFrom))
        {
            return custom_button_draw(&window, (*pnm).hwndFrom, lparam as LPNMCUSTOMDRAW);
        }

    } else if msg == WM_CONTEXTMENU {
        let menu = CreatePopupMenu();
        InsertMenuW(menu, -1i32 as u32, MF_BYPOSITION | MF_STRING | MF_ENABLED,
                    ContextMenu::Reset as usize, as_wstr("Reset").as_ptr());
        InsertMenuW(menu, -1i32 as u32, MF_BYPOSITION | MF_SEPARATOR,
                    0, null_mut());
        InsertMenuW(menu, -1i32 as u32, MF_BYPOSITION | MF_STRING | MF_ENABLED,
                    ContextMenu::Exit as usize, as_wstr("Exit").as_ptr());
        let selection = TrackPopupMenu(
            menu,
            TPM_RETURNCMD | TPM_TOPALIGN | TPM_LEFTALIGN,
            GET_X_LPARAM(lparam),
            GET_Y_LPARAM(lparam),
            0,
            hwindow,
            null_mut()
        );
        if selection == ContextMenu::Exit as BOOL {
            PostQuitMessage(0);
        } else if selection == ContextMenu::Reset as BOOL {
            let window = std::mem::transmute::<isize, &mut Window>(GetWindowLongPtrW(hwindow, GWLP_USERDATA));
            if window.handle.is_null() { PostQuitMessage(0); }
            window.robo_buttons
                .iter()
                .chain(window.item_buttons.iter())
                .for_each(|hb| {winuser::SendMessageW(*hb, BM_SETCHECK, BST_UNCHECKED, 0);});
        }
    }

    DefWindowProcW(hwindow, msg, wparam, lparam)
}

fn custom_button_draw(window: &Window, hwnd: HWND, nmc: LPNMCUSTOMDRAW) -> LRESULT
{
    use winapi::shared::windef::{
        RECT,
        HGDIOBJ,
    };
    use winapi::um::commctrl::{
        CDDS_PREERASE,
        CDRF_SKIPDEFAULT,
    };
    use winapi::um::winuser::{
        FillRect,
        GetClientRect,
        SendMessageW,
        BM_GETCHECK,
        BST_CHECKED,
    };
    use winapi::um::wingdi::{
        CreatePatternBrush,
        DeleteObject,
    };
    unsafe {
        if (*nmc).dwDrawStage == CDDS_PREERASE {
            let mut rc = RECT {top: 0, left: 0, right: 0, bottom: 0};
            GetClientRect(hwnd, &mut rc);
            // Now check if it's checked
            let status = SendMessageW(hwnd, BM_GETCHECK, 0, 0) as usize;
            let collections = if window.robo_buttons.contains(&hwnd) {
                (&window.robo_buttons, &window.robo_images)
            } else if window.item_buttons.contains(&hwnd) {
                (&window.item_buttons, &window.item_images)
            } else {
                panic!("impossible");
            };
            let pos = collections.0.iter().position(|h| *h == hwnd).unwrap();

            let hbitmap = if status == BST_CHECKED { collections.1[pos].1 } else { collections.1[pos].0 };
            let hbrush = CreatePatternBrush(hbitmap);
            FillRect((*nmc).hdc, &rc, hbrush);
            DeleteObject(hbrush as HGDIOBJ);
            return CDRF_SKIPDEFAULT;
        }
    }
    0
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

#[allow(dead_code)] //allow it because this mainly exists for debugging purposes
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
    let mut window = Window::new();
    initialize_window(&mut window, APP_NAME, APP_NAME).expect("Failed to create window");
    layout_window(&mut window).expect("Failed to layout window");

    loop {
        if !handle_message( &mut window ) {
            break;
        }
    }
}
