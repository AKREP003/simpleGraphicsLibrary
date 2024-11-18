
use crate::{HEIGHT, WIDTH};
use crate::graphics::{GraphicObjects, Rend, Visual};
use std::thread;
use crate::DiComplex::ComplexObjects;
use crate::DiComplex::ComplexObjects::Polygon;
use crate::graphics::Surface::Flat;
use crate::graphics::Compile;
use crate::TriGraphics::TriObjects;


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

