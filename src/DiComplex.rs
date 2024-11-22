use std::f32::consts::PI;
use crate::DiComplex::ComplexObjects::ComplexTriangle;
use crate::{graphics, transitions};
use crate::graphics::{Compile, DiCoordinate, GraphicObjects, GraphicTriangle, Surface, Transformation};
use crate::graphics::GraphicObjects::Triangle;
use crate::transitions::Transformer;
use crate::TriGraphics::CartesianCoordinate;

#[derive(Clone, Debug)]
pub struct CompTriangle {

    sub_triangles : [Option<GraphicTriangle>; 2]

}


impl CompTriangle {

   pub fn construct(coords : &mut [DiCoordinate; 3], surface : Surface) -> ComplexObjects{

       let mut buffer = CompTriangle {
           sub_triangles: [None, None],

       };

       coords.sort_by(|a, b| (a.0).cmp(&b.0));

       if coords[0].0 == coords[1].0 || coords[1].0 == coords[2].0{

           buffer.sub_triangles[0] = GraphicTriangle::construct( *coords, *surface);

       } else {

           let line =  graphics::line_between_points(coords[0], coords[2]).unwrap();

           let next0 = (coords[1].0, graphics::crossing_point(coords[1].0 - coords[0].0, Some(line)).unwrap());

           buffer.sub_triangles[0] = GraphicTriangle::construct( [coords[0], coords[1], next0,], *surface);
           buffer.sub_triangles[1] = GraphicTriangle::construct([coords[2], coords[1], next0, ], *surface)


       }

       return ComplexTriangle(buffer);
   }

}

impl Compile for CompTriangle {
    fn compile(&self) -> Vec<GraphicObjects> {
        let mut buffer = Vec::new();

        for i in self.sub_triangles.iter() {

            if let Some(triangle) = i {

                buffer.append(&mut triangle.compile());

            }

        }

        return buffer;
    }
}


#[derive(Clone, Debug)]
pub enum ComplexObjects {

    ComplexTriangle(CompTriangle),

    Quadrangle(DiCoordinate, DiCoordinate, DiCoordinate, DiCoordinate, Surface),

    Polygon(u32, u32, DiCoordinate, Surface),

}

impl Transformation<DiCoordinate> for ComplexObjects {
    fn rotate(&mut self, trans: Transformer, pivot: DiCoordinate) {

        match self {
            ComplexTriangle(p1, p2, p3, _) => {

                *p1 = transitions::tri_to_di(transitions::rotate_coordinate(transitions::di_to_tri(*p1), trans, transitions::di_to_tri(pivot)));
                *p2 = transitions::tri_to_di(transitions::rotate_coordinate(transitions::di_to_tri(*p2), trans, transitions::di_to_tri(pivot)));
                *p3 = transitions::tri_to_di(transitions::rotate_coordinate(transitions::di_to_tri(*p3), trans, transitions::di_to_tri(pivot)));

            }
            ComplexObjects::Quadrangle(p1, p2, p3, p4, _) => {

                *p1 = transitions::tri_to_di(transitions::rotate_coordinate(transitions::di_to_tri(*p1), trans, transitions::di_to_tri(pivot)));
                *p2 = transitions::tri_to_di(transitions::rotate_coordinate(transitions::di_to_tri(*p2), trans, transitions::di_to_tri(pivot)));
                *p3 = transitions::tri_to_di(transitions::rotate_coordinate(transitions::di_to_tri(*p3), trans, transitions::di_to_tri(pivot)));
                *p4 = transitions::tri_to_di(transitions::rotate_coordinate(transitions::di_to_tri(*p4), trans, transitions::di_to_tri(pivot)));

            }
            ComplexObjects::Polygon(_, _, _, _) => {
                panic!("don't")
            }
        }

    }
}

impl Compile for ComplexObjects {
    fn compile(&self) -> Vec<GraphicObjects> {
        match self {

            ComplexObjects::ComplexTriangle(CompTriangle) => {

                CompTriangle.compile()

            },


            ComplexObjects::Quadrangle(coordinate1, coordinate2, coordinate3, coordinate4, surface) => {

                let mut sorted_by_x:Vec<DiCoordinate> = vec![*coordinate1, *coordinate2, *coordinate3, *coordinate4];

                sorted_by_x.sort_by(|a, b| (a.0).cmp(&b.0));

                let mut grouping = (
                    vec![*sorted_by_x.get(0).unwrap(), *sorted_by_x.get(1).unwrap()],
                    vec![*sorted_by_x.get(2).unwrap(), *sorted_by_x.get(3).unwrap()]

                );

                grouping.0.sort_by(|a, b| (a.1).cmp(&b.1));
                grouping.1.sort_by(|a, b| (a.1).cmp(&b.1));

                return vec![
                    Triangle(
                        grouping.0.get(0).unwrap().clone(),
                        grouping.0.get(1).unwrap().clone(),
                        grouping.1.get(0).unwrap().clone(),
                        (*surface).clone()

                    ),
                    Triangle(
                        grouping.1.get(1).unwrap().clone(),
                        grouping.0.get(1).unwrap().clone(),
                        grouping.1.get(0).unwrap().clone(),
                        (*surface).clone()

                    )
                ];



            }

            ComplexObjects::Polygon(n, radius, center, surface) => {

                let mut coordinates: Vec<DiCoordinate> = vec![];

                for i in 0 .. *n {

                    coordinates.push((
                        center.0 + (*radius as f32 * (i as f32 * (2.0 * PI / *n as f32)).cos()) as i32,
                        center.1 + (*radius as f32 * (i as f32 * (2.0 * PI / *n as f32)).sin()) as i32
                    ));
                }

                let mut triangles:Vec<GraphicObjects> = vec![
                    Triangle(coordinates.get(0).unwrap().clone(),
                                    coordinates.get(coordinates.len() - 1).unwrap().clone(),
                                    center.clone(),
                                    surface.clone())
                ];

                for i in 0 .. coordinates.len() - 1 {
                    triangles.push(Triangle(
                        coordinates.get(i).unwrap().clone(),
                        coordinates.get(i + 1).unwrap().clone(),
                        center.clone(),
                        surface.clone()
                    ))

                }

                return triangles;

            }
        }
    }
}
