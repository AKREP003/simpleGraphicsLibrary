use crate::graphics::{Colour, Compile, GraphicObjects, Surface};
use crate::DiComplex::ComplexObjects::{CTriangle, Qangle};
use crate::DiComplex::{ComplexTriangle, Quadrangle};
use crate::transitions::{rotate_coordinate, Transformation, Transformer};

pub type SphericalCoordinate = (f32, f32, f32);
pub type CartesianCoordinate = (i32, i32, i32);


fn spherical_to_cartesian(c: SphericalCoordinate) -> CartesianCoordinate {
    let (r, theta, phi) = c;
    let x = r * theta.cos() * phi.sin();
    let y = r * theta.sin() * phi.sin();
    let z = r * phi.cos();
    (x as i32, y as i32, z as i32)
}

#[derive(Clone, Copy, Debug)]
pub enum TriObjects {
    TriLine(CartesianCoordinate, CartesianCoordinate, Colour),
    TriTriangle(CartesianCoordinate, CartesianCoordinate, CartesianCoordinate, Surface),
    TriQuadrangle(CartesianCoordinate, CartesianCoordinate, CartesianCoordinate, CartesianCoordinate, Surface),
}


impl Compile for TriObjects {
    fn compile(&self) -> Vec<GraphicObjects> {

        match self {
            TriObjects::TriLine((x1,y1 , _), (x2,y2 , _), c) => {

                vec![GraphicObjects::Line((*x1, *y1), (*x2, *y2), *c)]

            },

            TriObjects::TriTriangle((x1, y1, _), (x2, y2, _), (x3, y3, _), c) => {

                let buffer = ComplexTriangle::construct(&mut [(*x1, *y1), (*x2, *y2), (*x3, *y3)], (*c).clone());

                return buffer.compile();

            },

            TriObjects::TriQuadrangle((x1, y1, _), (x2, y2, _), (x3, y3, _), (x4, y4, _), c) => {
                let buffer = Quadrangle::construct(&mut [(*x1, *y1), (*x2, *y2), (*x3, *y3), (*x4, *y4)], (*c).clone());

                let mut result = buffer.compile();

                return result;
            }
        }

    }

}

impl Transformation<CartesianCoordinate> for TriObjects {
    fn rotate(&mut self, trans: Transformer, pivot: CartesianCoordinate) {
        match self {
            TriObjects::TriLine(x, y, _) => {
                *x = rotate_coordinate(*x, trans, pivot);
                *y = rotate_coordinate(*y, trans, pivot);
            },

            TriObjects::TriTriangle(x, y, z, _) => {
                *x = rotate_coordinate(*x, trans, pivot);
                *y = rotate_coordinate(*y, trans, pivot);
                *z = rotate_coordinate(*z, trans, pivot);
            },

            TriObjects::TriQuadrangle(x, y, z, w, _) => {
                *x = rotate_coordinate(*x, trans, pivot);

                *y = rotate_coordinate(*y, trans, pivot);

                *z = rotate_coordinate(*z, trans, pivot);
                *w = rotate_coordinate(*w, trans, pivot);

            }
        }
    }

}
