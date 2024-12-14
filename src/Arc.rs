use crate::Arc::Arche::Tri;
use crate::DiComplex::{ComplexObjects, ComplexTriangle, Quadrangle};
use crate::graphics::{Compile, GraphicObjects};
use crate::transitions::{Transformation, Transformer};
use crate::TriComplex::{Rectprism, TriComplexes};
use crate::TriGraphics::{CartesianCoordinate, TriObjects, TriQuadrangle, TriTriangle};
use crate::TriGraphics::TriObjects::TriTring;

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
