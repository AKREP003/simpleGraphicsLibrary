use std::thread;

use crate::{HEIGHT, WIDTH};
use crate::Arc::Arche;
use crate::DiComplex::{ComplexObjects, ComplexTriangle, Quadrangle};
use crate::DiComplex::ComplexObjects::Polygon;
use crate::graphics::{GraphicObjects, Rend, Visual};
use crate::graphics::Compile;
use crate::graphics::Surface::Flat;
use crate::Arc::Arche::Tri;
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

pub struct State {
    pub(crate) objects: Vec<Arche>,

    pub canvas: Option<Visual>,

}

