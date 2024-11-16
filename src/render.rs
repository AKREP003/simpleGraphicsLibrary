
use crate::{HEIGHT, WIDTH};
use crate::objects::{ComplexObjects, GraphicObjects, Rend, Visual};
use std::thread;
use crate::objects::ComplexObjects::Polygon;
use crate::objects::Surface::Flat;
use crate::objects::Compile;



pub(crate) fn draw_gradient(pixels: &mut Vec<u8>, objects: Vec<ComplexObjects>) {

    unsafe {

        if objects.len() == 0 {
            //thread::yield_now();
            return
        }

        let mut objectBuffer:Vec<GraphicObjects> = vec![];

        for object in objects.iter() {

            objectBuffer.append(&mut object.compile());


        }

        for graphic in objectBuffer {

            graphic.rend(pixels);

        }

    }


}




pub struct State {

    pub(crate) objects:Vec<ComplexObjects>,

    pub canvas:Option<Visual>,


}

