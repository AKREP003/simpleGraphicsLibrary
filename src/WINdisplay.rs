use std::{ptr, thread};
use std::ffi::CString;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::LPCSTR;
use winapi::shared::windef::{HDC, HWND};
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::processthreadsapi::{CreateProcessA, CreateThread, GetCurrentThreadId};
use winapi::um::wingdi::{BI_RGB, BITMAPINFO, BITMAPINFOHEADER, RGBQUAD, SetDIBitsToDevice};
use winapi::um::winuser::{AttachThreadInput, CreateWindowExA, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, DefWindowProcA, DispatchMessageA, GetDC, GetKeyboardState, IDC_ARROW, LoadCursorW, MSG, PostQuitMessage, RegisterClassA, TranslateMessage, WM_DESTROY, WM_PAINT, WNDCLASSA, WS_OVERLAPPEDWINDOW, WS_VISIBLE};
use crate::Arc::Arche;

use crate::DiComplex::ComplexObjects;
use crate::render::State;
use crate::TriGame::camera_transition;

pub const WIDTH: i32 = 900;
pub const HEIGHT: i32 = 900;

type DrawCallback = fn(&mut Vec<u8>, objects: Vec<Arche>);

type EventLoop = unsafe fn() -> Option<State>;

static mut DRAW_CALLBACK: Option<DrawCallback> = None;

static mut EVENTLOOP: Option<EventLoop> = None;

static mut PIXEL_BUFFER: Vec<u8> = vec![];

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            let hdc: HDC = GetDC(hwnd);


            let rgbq: RGBQUAD = RGBQUAD {
                rgbBlue: 0,
                rgbGreen: 0,
                rgbRed: 0,
                rgbReserved: 0,
            };

            // Set up the bitmap info header
            let mut bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: WIDTH,
                    biHeight: -HEIGHT, // Negative height to indicate a top-down DIB
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [rgbq; 1],
            };

            // Draw pixels to the window using SetDIBitsToDevice
            SetDIBitsToDevice(
                hdc,
                0,
                0,
                WIDTH as u32,
                HEIGHT as u32,
                0,
                0,
                0,
                HEIGHT as u32,
                PIXEL_BUFFER.as_ptr() as *const _,
                &bmi,
                winapi::um::wingdi::DIB_RGB_COLORS,
            );

            0
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcA(hwnd, msg, w_param, l_param),
    }
}

unsafe extern "system" fn render_loop(_: *mut winapi::ctypes::c_void) -> u32 {
    while true {
        if let Some(callback) = DRAW_CALLBACK {
            if let Some(s) = EVENTLOOP && let Some(state) = s() {
                if let Some(v) = state.canvas {
                    PIXEL_BUFFER = v;
                }

                callback(&mut PIXEL_BUFFER, state.objects);
            }
        }
    }

    return 0;
}


// Wrapper function to set up and run the window loop
pub fn run_window(draw_callback: DrawCallback, event_loop: EventLoop) {
    unsafe {
        let mut renderId: u32 = 0;

        CreateThread(
            ptr::null_mut(),
            0,
            Some(render_loop),
            ptr::null_mut(),
            0,
            &mut renderId as *mut u32,
        );

        AttachThreadInput(GetCurrentThreadId(), renderId, 1);


        PIXEL_BUFFER = vec![0u8; (WIDTH * HEIGHT * 4) as usize];

        let h_instance = GetModuleHandleA(ptr::null());
        let class_name = CString::new("window").unwrap();

        // Register the window class
        let wnd_class = WNDCLASSA {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc), // Register the window_proc callback
            hInstance: h_instance,
            lpszClassName: class_name.as_ptr() as LPCSTR,
            hCursor: LoadCursorW(ptr::null_mut(), IDC_ARROW),
            ..std::mem::zeroed()
        };

        RegisterClassA(&wnd_class);

        EVENTLOOP = Some(event_loop);

        // Set the draw callback in the static variable
        DRAW_CALLBACK = Some(draw_callback);

        // Create the window
        let hwnd = CreateWindowExA(
            0,
            class_name.as_ptr(),
            CString::new("Direct Pixel Drawing in Rust").unwrap().as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            WIDTH,
            HEIGHT,
            ptr::null_mut(),
            ptr::null_mut(),
            h_instance,
            ptr::null_mut(),
        );


        // Run the message loop
        let mut msg: MSG = std::mem::zeroed();
        while winapi::um::winuser::PeekMessageA(&mut msg, ptr::null_mut(), 0, 400, 1) > 0 {
            camera_transition();

            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
}





