use std::f32::consts::PI;
use crate::DiComplex::ComplexObjects::ComplexTriangle;
use crate::graphics;
use crate::graphics::{Compile, DiCoordinate, GraphicObjects, Surface, Transformation};
use crate::graphics::GraphicObjects::Triangle;
use crate::TriGraphics::CartesianCoordinate;

#[derive(Clone, Debug, Copy)]
pub enum ComplexObjects {

    ComplexTriangle(DiCoordinate, DiCoordinate, DiCoordinate, Surface),

    Quadrangle(DiCoordinate, DiCoordinate, DiCoordinate, DiCoordinate, Surface),

    Polygon(u32, u32, DiCoordinate, Surface),

}

pub(crate) type Transformer = ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64));

pub fn matrix_mult(
    vector: (i32, i32, i32),
    matrix: ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64))
) -> (i32, i32, i32) {

     (
        (vector.0 as f64 * matrix.0 .0 + vector.1 as f64 * matrix.1 .0 + vector.2 as f64 * matrix.2 .0).ceil() as i32,
        (vector.0 as f64 * matrix.0 .1 + vector.1 as f64 * matrix.1 .1 + vector.2 as f64 * matrix.2 .1).ceil() as i32,
        (vector.0 as f64* matrix.0 .2 + vector.1 as f64 * matrix.1 .2 + vector.2 as f64 * matrix.2 .2).ceil() as i32,
    )
}

fn di_to_tri((x, y): DiCoordinate) -> CartesianCoordinate { (x, y, 0) }

fn tri_to_di((x, y, _) : CartesianCoordinate) -> DiCoordinate {(x, y)}

pub fn matrix_sub((x1, y1, z1) : CartesianCoordinate, (x2, y2, z2) : CartesianCoordinate) -> CartesianCoordinate {(x1 - x2, y1 - y2, z1 - z2)}

pub fn matrix_add((x1, y1, z1) : CartesianCoordinate, (x2, y2, z2) : CartesianCoordinate) -> CartesianCoordinate {(x1 + x2, y1 + y2, z1 + z2)}

pub fn transform_coordinate(x:CartesianCoordinate, t_matrix:Transformer, pivot:CartesianCoordinate) -> CartesianCoordinate {

    matrix_add(pivot, matrix_mult(matrix_sub(x, pivot.clone() ), t_matrix) )

}

impl Transformation<DiCoordinate> for ComplexObjects {
    fn rotate(&mut self, trans: Transformer, pivot: DiCoordinate) {

        match self {
            ComplexTriangle(p1, p2, p3, _) => {

                *p1 = tri_to_di(transform_coordinate(di_to_tri(*p1), trans, di_to_tri(pivot)));
                *p2 = tri_to_di(transform_coordinate(di_to_tri(*p2), trans, di_to_tri(pivot)));
                *p3 = tri_to_di(transform_coordinate(di_to_tri(*p3), trans, di_to_tri(pivot)));

            }
            ComplexObjects::Quadrangle(p1, p2, p3, p4, _) => {

                *p1 = tri_to_di(transform_coordinate(di_to_tri(*p1), trans, di_to_tri(pivot)));
                *p2 = tri_to_di(transform_coordinate(di_to_tri(*p2), trans, di_to_tri(pivot)));
                *p3 = tri_to_di(transform_coordinate(di_to_tri(*p3), trans, di_to_tri(pivot)));
                *p4 = tri_to_di(transform_coordinate(di_to_tri(*p4), trans, di_to_tri(pivot)));

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

            ComplexObjects::ComplexTriangle(coordinate1, coordinate2, coordinate3, surface) => {

                let mut coords = vec![*coordinate1, *coordinate2, *coordinate3];

                coords.sort_by(|a, b| (a.0).cmp(&b.0));

                let coordinates = (*coords.get(0).unwrap(), *coords.get(1).unwrap(), *coords.get(2).unwrap());

                if coordinates.0.0 == coords.get(1).unwrap().0 || coordinates.1.0 == coordinates.2.0{

                    return vec![Triangle(coordinate1.clone(), coordinate2.clone(), coordinate3.clone(), surface.clone())];


                } else {

                    let line =  graphics::line_between_points(coordinates.0, coordinates.2).unwrap();

                    let next0 = (coordinates.1.0, graphics::crossing_point(coordinates.1.0 - coordinates.0.0, Some(line)).unwrap());

                    return vec![
                        Triangle(
                            coordinates.0,
                            coordinates.1,
                            next0,
                            (*surface).clone()
                        ),
                        Triangle(
                            coordinates.2,
                            coordinates.1,
                            next0,
                            (*surface).clone()
                        )
                    ];


                }

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
