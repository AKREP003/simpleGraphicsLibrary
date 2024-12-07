use crate::graphics::{Colour, Compile, DiCoordinate, GraphicObjects, Surface};
use crate::DiComplex::ComplexObjects::{CTriangle, Qangle};
use crate::DiComplex::{ComplexTriangle, Quadrangle};
use crate::render::Arche;
use crate::render::Arche::Tri;
use crate::transitions::{rotate_coordinate, Transformation, Transformer, tri_to_di};
use crate::TriGraphics::TriObjects::{ TriTring};

pub type SphericalCoordinate = (f32, f32, f32);
pub type CartesianCoordinate = (f64, f64, f64);


fn spherical_to_cartesian(c: SphericalCoordinate) -> CartesianCoordinate {
    let (r, theta, phi) = c;
    let x = r * theta.cos() * phi.sin();
    let y = r * theta.sin() * phi.sin();
    let z = r * phi.cos();
    (x.into() , y.into() , z.into() )
}

#[derive(Clone, Copy, Debug)]
pub struct TriTriangle {

    di : ComplexTriangle,
    coords : [CartesianCoordinate; 3],
    surface : Surface,
    pub center : CartesianCoordinate,

}

impl TriTriangle {

    pub fn construct(coords: &mut [CartesianCoordinate; 3], surface: Surface) -> Self {

        let mut buffer:[DiCoordinate; 3] = [tri_to_di(coords[0]), tri_to_di(coords[1]), tri_to_di(coords[2])];

        let di = ComplexTriangle::construct(&mut buffer, surface.clone());
        let mut buffer = TriTriangle {
            di,
            coords: *coords,
            surface,
            center : (0.0,0.0,0.0),
        };

        buffer.center = buffer.get_center();

        return buffer;
    }

    pub fn get_center(&self) -> CartesianCoordinate {
        let mut x:f64 = 0.0;
        let mut y:f64 = 0.0;
        let mut z:f64 = 0.0;

        self.coords.iter().for_each(|d|
            {
                y += (d.1 as f64 / 3.0);
                x += (d.0 as f64/ 3.0);
                z += (d.2 as f64/ 3.0);
            }
        );


        (x, y,  z)

    }

}

impl Compile for TriTriangle {
    fn compile(&self) -> Vec<GraphicObjects> {
        return self.di.compile();
    }
}

impl Transformation<CartesianCoordinate> for TriTriangle {
    fn rotate(&mut self, trans: Transformer, pivot: CartesianCoordinate) {
        for i in 0..3 {
            self.coords[i] = rotate_coordinate(self.coords[i], trans, pivot);
        }
        self.di = ComplexTriangle::construct(&mut [
            tri_to_di(self.coords[0]),
            tri_to_di(self.coords[1]),
            tri_to_di(self.coords[2])
        ], self.surface.clone());

        self.center = self.get_center();

    }
}

#[derive(Clone, Copy, Debug)]
pub struct TriQuadrangle {
    di : Quadrangle,
    coords : [CartesianCoordinate; 4],
    surface : Surface,
    center : CartesianCoordinate,
}

impl TriQuadrangle {
    pub fn construct(coords: &mut [CartesianCoordinate; 4], surface: Surface) -> Self {

        let mut buffer:[DiCoordinate; 4] = [tri_to_di(coords[0]), tri_to_di(coords[1]), tri_to_di(coords[2]), tri_to_di(coords[3])];

        let di = Quadrangle::construct(&mut buffer, surface.clone());

        let mut buffer = TriQuadrangle {
            di,
            coords: *coords,
            surface,
            center : (0.0, 0.0, 0.0),
        };
        buffer.center = buffer.get_center();

        return buffer;
    }

    pub fn get_center(&self) -> CartesianCoordinate {
        let mut x:f64 = 0.0;
        let mut y:f64 = 0.0;
        let mut z:f64 = 0.0;

        self.coords.iter().for_each(|d|
            {
                y += (d.1 as f64 / 4.0);
                x += (d.0 as f64/ 4.0);
                z += (d.2 as f64/ 4.0);
            }
        );


        (x, y,  z)

    }

}

impl Compile for TriQuadrangle {
    fn compile(&self) -> Vec<GraphicObjects> {

        return self.di.compile();
    }
}

impl Transformation<CartesianCoordinate> for TriQuadrangle {
    fn rotate(&mut self, trans: Transformer, pivot: CartesianCoordinate) {
        for i in 0..4 {
            self.coords[i] = rotate_coordinate(self.coords[i], trans, pivot);
        }
        self.di = Quadrangle::construct(&mut [
            tri_to_di(self.coords[0]),
            tri_to_di(self.coords[1]),
            tri_to_di(self.coords[2]),
            tri_to_di(self.coords[3])
        ], self.surface.clone());
        self.center = self.get_center();
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TriObjects {
    TriLine(CartesianCoordinate, CartesianCoordinate, Colour),
    TriTring(TriTriangle),
    TriQuad(TriQuadrangle),
}


impl Compile for TriObjects {
    fn compile(&self) -> Vec<GraphicObjects> {

        match self {
            TriObjects::TriLine((x1,y1 , _), (x2,y2 , _), c) => {

                vec![GraphicObjects::Line((x1.round() as i32, y1.round() as i32), (x2.round() as i32, y2.round() as i32), *c)]

            },

            TriObjects::TriTring(t) => {

                return t.compile();

            },

            TriObjects::TriQuad(q) => {

                return q.compile();
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

            TriObjects::TriTring(t) => {
                t.rotate(trans, pivot)
            },

            TriObjects::TriQuad(q) => {
                q.rotate(trans, pivot)

            }
        }
    }

}
