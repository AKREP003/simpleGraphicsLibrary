
use crate::{State, WIDTH, HEIGHT, STATE};
use crate::objects::Rend;


pub(crate) fn draw_gradient(pixels: &mut Vec<u8>) {

    unsafe {



        if STATE.objects.len() == 0 {
            return
        }

        for object in STATE.objects.iter() {



            object.rend(pixels, &STATE)

        }

        STATE.objects=vec![];



    }


}

