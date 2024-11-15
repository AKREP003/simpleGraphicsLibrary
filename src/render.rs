
use crate::{HEIGHT, WIDTH};
use crate::objects::{Objects, Rend, Visual};
use std::thread;
use crate::objects::Objects::Polygon;
use crate::objects::Surface::Flat;




pub(crate) fn draw_gradient(pixels: &mut Vec<u8>, objects: Vec<Objects>) {

    unsafe {

        if objects.len() == 0 {
            //thread::yield_now();
            return
        }

        for object in objects.iter() {

            object.rend(pixels);


        }



    }


}




pub struct State {

    pub(crate) objects:Vec<Objects>,

    pub canvas:Option<Visual>,


}

