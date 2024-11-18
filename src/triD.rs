use crate::objects::{Colour, Compile, GraphicObjects, Surface, transform_coordinate, Transformation, Transformer};
use crate::objects::ComplexObjects::ComplexTriangle;

pub type SphericalCoordinate = (f32, f32, f32);
pub type CartesianCoordinate = (i32, i32, i32);


fn spherical_to_cartesian(c: SphericalCoordinate) -> CartesianCoordinate {
    let (r, theta, phi) = c;
    let x = r * theta.cos() * phi.sin();
    let y = r * theta.sin() * phi.sin();
    let z = r * phi.cos();
    (x as i32, y as i32, z as i32)
}

#[derive(Clone, Copy)]
pub enum TriObjects {
    TriLine(CartesianCoordinate, CartesianCoordinate, Colour),
    TriTriangle(CartesianCoordinate, CartesianCoordinate, CartesianCoordinate, Surface)
}


impl Compile for TriObjects {
    fn compile(&self) -> Vec<GraphicObjects> {

        match self {
            TriObjects::TriLine((x1,y1 , _), (x2,y2 , _), c) => {

                vec![GraphicObjects::Line((*x1, *y1), (*x2, *y2), *c)]

            },

            TriObjects::TriTriangle((x1, y1, _), (x2, y2, _), (x3, y3, _), c) => {

                let buffer = ComplexTriangle((*x1, *y1), (*x2, *y2), (*x3, *y3), (*c).clone());

                return buffer.compile();

            }
        }

    }

}

impl Transformation<CartesianCoordinate> for TriObjects {

    fn rotate(&mut self, trans: Transformer, pivot: CartesianCoordinate) {

        match self {
            TriObjects::TriLine(x, y, _) => {

                *x = transform_coordinate(*x, trans, pivot);
                *y = transform_coordinate(*y, trans, pivot);

            },

            TriObjects::TriTriangle(x, y, z, _) => {
                *x = transform_coordinate(*x, trans, pivot);
                *y = transform_coordinate(*y, trans, pivot);
                *z = transform_coordinate(*z, trans, pivot);
            }
        }

    }

}
