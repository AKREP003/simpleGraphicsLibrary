
use crate::{HEIGHT, WIDTH};
use crate::objects::{ComplexObjects, GraphicObjects, Rend, Visual};
use std::thread;
use crate::objects::ComplexObjects::Polygon;
use crate::objects::Surface::Flat;
use crate::objects::Compile;
use crate::triD::TriObjects;


pub(crate)  fn draw_gradient(pixels: &mut Vec<u8>, objects: Vec<Arche>) {

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

pub enum Arche {

    Tri(TriObjects),
    Di(ComplexObjects),
    Graphic(GraphicObjects)

}

impl Compile for Arche {
    fn compile(&self) -> Vec<GraphicObjects> {
        match self {
            Arche::Tri(tri) => tri.compile(),
            Arche::Di(di) => di.compile(),
            Arche::Graphic(gr) => vec![gr.clone()]
        }
    }
}

pub struct State {

    pub(crate) objects:Vec<Arche>,

    pub canvas:Option<Visual>,


}

