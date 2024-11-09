
use crate::{State, WIDTH, HEIGHT, STATE};
use crate::objects::Rend;


pub(crate) fn draw_gradient(pixels: &mut Vec<u8>) {

    unsafe {



        let mut index = 0;

        for object in STATE.objects.iter() {

            for data in object.rend().iter() {

                pixels[index.clone()] = data.clone();

                index += 1;


            }

        }

    }


}

