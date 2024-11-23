
use crate::{HEIGHT, WIDTH};
use crate::graphics::{GraphicObjects, Rend, Visual};
use std::thread;
use crate::DiComplex::ComplexObjects;
use crate::DiComplex::ComplexObjects::Polygon;
use crate::graphics::Surface::Flat;
use crate::graphics::Compile;
use crate::transitions::{Transformation, Transformer};
use crate::TriComplex::TriComplexes;
use crate::TriGraphics::{CartesianCoordinate, TriObjects};

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

#[derive(Clone, Copy, Debug)]
pub enum Arche {

    TriC(TriComplexes),
    Tri(TriObjects),
    Di(ComplexObjects),
    Graphic(GraphicObjects),
    Null

}

impl Compile for Arche {
    fn compile(&self) -> Vec<GraphicObjects> {
        match self {
            Arche::Tri(tri) => tri.compile(),
            Arche::Di(di) => di.compile(),
            Arche::Graphic(gr) => vec![gr.clone()],
            Arche::TriC(tri) => tri.compile(),
            Arche::Null => {vec![]}
        }
    }
}

impl Transformation<CartesianCoordinate> for Arche{
    fn rotate(&mut self, trans: Transformer, pivot: CartesianCoordinate) {

        match self {
            Arche::Tri(tri) => tri.rotate(trans, pivot),
            Arche::Di(di) => di.rotate(trans, pivot),
            Arche::Graphic(gr) => {}
            Arche::TriC(tri) => tri.rotate(trans, pivot),
            Arche::Null => {}
        }
    }
}

pub struct State {

    pub(crate) objects:Vec<Arche>,

    pub canvas:Option<Visual>,


}

