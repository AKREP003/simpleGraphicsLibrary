use std::thread;

use crate::{HEIGHT, WIDTH};
use crate::DiComplex::{ComplexObjects, ComplexTriangle, Quadrangle};
use crate::DiComplex::ComplexObjects::Polygon;
use crate::graphics::{GraphicObjects, Rend, Visual};
use crate::graphics::Compile;
use crate::graphics::Surface::Flat;
use crate::render::Arche::Tri;
use crate::transitions::{Transformation, Transformer};
use crate::TriComplex::{Rectprism, TriComplexes};
use crate::TriGraphics::{CartesianCoordinate, TriObjects, TriQuadrangle, TriTriangle};
use crate::TriGraphics::TriObjects::TriTring;

pub(crate) fn draw_gradient(pixels: &mut Vec<u8>, objects: Vec<Arche>) {
    unsafe {
        if objects.len() == 0 {
            //thread::yield_now();
            return;
        }

        let mut objectBuffer: Vec<GraphicObjects> = vec![];

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
    Null,
}

impl From<TriTriangle> for Arche {
    fn from(value: TriTriangle) -> Self {
        Tri(TriTring(value))
    }
}

impl From<ComplexTriangle> for Arche {
    fn from(value: ComplexTriangle) -> Self {
        Arche::Di(ComplexObjects::CTriangle(value))
    }
}

impl From<Quadrangle> for Arche {
    fn from(value: Quadrangle) -> Self {
        Arche::Di(ComplexObjects::Qangle(value))
    }
}

impl From<TriQuadrangle> for Arche {
    fn from(value: TriQuadrangle) -> Self {
        Arche::Tri(TriObjects::TriQuad(value))
    }
}

impl From<Rectprism> for Arche {
    fn from(value: Rectprism) -> Self {
        Arche::TriC(TriComplexes::RectangularPrism(value))
    }
}

impl Compile for Arche {
    fn compile(&self) -> Vec<GraphicObjects> {
        match self {
            Arche::Tri(tri) => tri.compile(),
            Arche::Di(di) => di.compile(),
            Arche::Graphic(gr) => vec![gr.clone()],
            Arche::TriC(tri) => tri.compile(),
            Arche::Null => { vec![] }
        }
    }
}

impl Transformation<CartesianCoordinate> for Arche {
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
    pub(crate) objects: Vec<Arche>,

    pub canvas: Option<Visual>,

}

