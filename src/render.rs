
use crate::{State, WIDTH, HEIGHT, STATE};
use crate::objects::Rend;
use std::thread;
use std::time::Duration;


pub(crate) fn draw_gradient(pixels: &mut Vec<u8>) {

    thread::sleep(Duration::from_millis(7)); //144 fps

    unsafe {

        if STATE.objects.len() == 0 {
            thread::yield_now();
            return
        }

        for object in STATE.objects.iter() {



            object.rend(pixels, &STATE)

        }

        STATE.objects=vec![];



    }


}

