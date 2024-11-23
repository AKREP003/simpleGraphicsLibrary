use std::f32::consts::PI;
use crate::DiComplex::ComplexObjects::{CTriangle, Qangle};
use crate::{graphics, transitions};
use crate::graphics::{Compile, DiCoordinate, GraphicObjects, GraphicTriangle, Surface};
use crate::graphics::GraphicObjects::Triangle;
use crate::render::Arche;
use crate::render::Arche::Di;
use crate::transitions::{di_to_tri, rotate_coordinate, Transformation, Transformer, tri_to_di};
use crate::TriGraphics::CartesianCoordinate;

#[derive(Clone, Debug, Copy)]
pub struct ComplexTriangle {

    sub_triangles : [Option<GraphicTriangle>; 2],

    coords : [DiCoordinate; 3],

}


impl ComplexTriangle {

   pub fn construct(coords : &mut [DiCoordinate; 3], surface : Surface) -> ComplexTriangle{

       let mut buffer = ComplexTriangle {
           sub_triangles: [None, None],
            coords : *coords
       };

       coords.sort_by(|a, b| (a.0).cmp(&b.0));

       if coords[0].0 == coords[1].0 || coords[1].0 == coords[2].0{

           buffer.sub_triangles[0] = Some(GraphicTriangle::construct( *coords, surface));

       } else {

           let line =  graphics::line_between_points(coords[0], coords[2]).unwrap();

           let next0 = (coords[1].0, graphics::crossing_point(coords[1].0 - coords[0].0, Some(line)).unwrap());

           buffer.sub_triangles[0] = Some(GraphicTriangle::construct( [coords[0], coords[1], next0,], surface));
           buffer.sub_triangles[1] = Some(GraphicTriangle::construct([coords[2], coords[1], next0, ], surface));


       }

       return buffer;
   }

   pub fn into(self) -> Arche {
       return Di(CTriangle(self));
   }
}

impl Compile for ComplexTriangle {
    fn compile(&self) -> Vec<GraphicObjects> {

        return self.sub_triangles.into_iter().filter_map(|x| x).map(|x| Triangle(x)).collect::<Vec<GraphicObjects>>();
    }
}

impl Transformation<CartesianCoordinate> for ComplexTriangle {
    fn rotate(&mut self, trans: Transformer, pivot: CartesianCoordinate) {

        *self = ComplexTriangle::construct(&mut self.coords.map(|c| tri_to_di(rotate_coordinate(di_to_tri(c), trans, pivot))), self.sub_triangles[0].unwrap().surf.clone());



    }
}

#[derive(Clone, Debug, Copy)]
pub struct  Quadrangle {

    side : [ComplexTriangle; 2]

}

impl Quadrangle {

    pub fn construct(coords : &mut [DiCoordinate; 4], surface : Surface) -> ComplexObjects {

        coords.sort_by(|a, b| (a.0).cmp(&b.0));

        let mut grouping = (
            [*coords.get(0).unwrap(), *coords.get(1).unwrap()],
            [*coords.get(2).unwrap(), *coords.get(3).unwrap()]

        );

        grouping.0.sort_by(|a, b| (a.1).cmp(&b.1));
        grouping.1.sort_by(|a, b| (a.1).cmp(&b.1));

        return Qangle(Quadrangle {

            side: [
                 ComplexTriangle::construct(&mut [
                    grouping.0.get(0).unwrap().clone(),
                    grouping.0.get(1).unwrap().clone(),
                    grouping.1.get(0).unwrap().clone()], surface.clone()),
                 (ComplexTriangle::construct(&mut [
                    grouping.1.get(1).unwrap().clone(),
                    grouping.0.get(1).unwrap().clone(),
                    grouping.1.get(0).unwrap().clone()], surface.clone()))
            ]
        });

    }

    pub fn into(self) -> ComplexObjects {
        return Qangle(self);
    }

}

impl Transformation<CartesianCoordinate> for Quadrangle {
    fn rotate(&mut self, trans: Transformer, pivot: CartesianCoordinate) {

        self.side.iter_mut().for_each(|x| x.rotate(trans, pivot));

    }
}

impl Compile for Quadrangle {
    fn compile(&self) -> Vec<GraphicObjects> {
        let mut buffer = Vec::new();

        for i in self.side.iter() {

            buffer.append(&mut i.compile());

        }

        return buffer;
    }
}

#[derive(Clone, Debug, Copy)]
pub enum ComplexObjects {

    CTriangle(ComplexTriangle),

    Qangle(Quadrangle),

    Polygon(u32, u32, DiCoordinate, Surface),

    Null

}

impl Transformation<CartesianCoordinate> for ComplexObjects {
    fn rotate(&mut self, trans: Transformer, pivot: CartesianCoordinate) {

        match self {
            CTriangle(t) => {

                t.rotate(trans, pivot)

            }
            ComplexObjects::Qangle(t) => {

                t.rotate(trans, pivot)

            }
            ComplexObjects::Polygon(_, _, _, _) => {
                panic!("don't")
            }

            ComplexObjects::Null => {
                panic!("don't")
            }
        }

    }
}

impl Compile for ComplexObjects {
    fn compile(&self) -> Vec<GraphicObjects> {
        match self {

            ComplexObjects::Null => {
                return vec![];
            }

            ComplexObjects::CTriangle(CompTriangle) => {

                CompTriangle.compile()

            },


            ComplexObjects::Qangle(q) => {

                q.compile()

            }

            ComplexObjects::Polygon(n, radius, center, surface) => {

                let mut coordinates: Vec<DiCoordinate> = vec![];

                for i in 0 .. *n {

                    coordinates.push((
                        center.0 + (*radius as f32 * (i as f32 * (2.0 * PI / *n as f32)).cos()) as i32,
                        center.1 + (*radius as f32 * (i as f32 * (2.0 * PI / *n as f32)).sin()) as i32
                    ));
                }

                let mut triangles:Vec<GraphicObjects> =
                    ComplexTriangle::construct(&mut [coordinates.get(0).unwrap().clone(),
                                    coordinates.get(coordinates.len() - 1).unwrap().clone(),
                                    center.clone()],
                                    surface.clone()).compile();


                for i in 0 .. coordinates.len() - 1 {
                    triangles.append(&mut ComplexTriangle::construct(
                        &mut [coordinates.get(i).unwrap().clone(),
                            coordinates.get(i + 1).unwrap().clone(),
                            center.clone()],
                        surface.clone()
                    ).compile())

                }

                return triangles;

            }
        }
    }
}
