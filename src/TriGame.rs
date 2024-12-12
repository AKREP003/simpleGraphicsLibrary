use winapi::shared::windef::POINT;
use winapi::um::winuser::{GetCursorPos, GetKeyboardState};

use crate::camera::Camera;
use crate::WINdisplay::{HEIGHT, WIDTH};

pub static mut cam: Camera = Camera { position: (((WIDTH / 2)) as f64, ((HEIGHT / 2)) as f64, 0.0), orientation: (0.0, 0.0, 90.0) };


pub unsafe fn camera_transition() {
    let mut pointer: POINT = POINT {
        x: 0,
        y: 0,
    };

    GetCursorPos(&mut pointer);

    cam.orientation.0 = -((((pointer.x as f64) - 960.0) / 1920.0) * 360.0);
    cam.orientation.1 = -((((pointer.y as f64) - 540.0) / 1080.0) * 180.0);

    let x_progress = 10.0 * (cam.orientation.0.to_radians()).cos();
    let y_progress = 10.0 * (cam.orientation.0.to_radians()).sin();


    let mut k_st = [0u8; 256];
    GetKeyboardState(k_st.as_mut_ptr());

    if k_st[0x41] == 128 || k_st[0x41] == 129 { //A
        cam.position.1 -= x_progress;
        cam.position.2 -= y_progress;
    }
    if k_st[0x44] == 128 || k_st[0x41] == 129 { //D
        cam.position.1 += x_progress;
        cam.position.2 += y_progress;
    }
    if k_st[0x57] == 128 || k_st[0x57] == 129 { //W
        cam.position.1 -= y_progress;
        cam.position.2 += x_progress;
    }
    if k_st[0x53] == 128 || k_st[0x53] == 129 { //S
        cam.position.1 += y_progress;
        cam.position.2 -= x_progress;
    }
}