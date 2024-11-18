use std::cmp::{max, min};
use std::f32::consts::PI;
use std::intrinsics::{ceilf32, floorf32, roundf32};
use std::process::exit;
use crate::{HEIGHT, WIDTH};
use crate::objects::ComplexObjects::ComplexTriangle;
use crate::objects::GraphicObjects::Triangle;
use crate::objects::Surface::Flat;
use crate::render::State;
use crate::triD::CartesianCoordinate;

pub type Visual = Vec<u8>;


pub trait Rend {
    fn rend(&self, rendered: &mut Visual);
}

pub trait Compile {
    fn compile(&self) -> Vec<GraphicObjects>;
}

pub type Colour = (u8, u8, u8, u8);
pub type DiCoordinate = (i32, i32);


fn indexify(c:&DiCoordinate) -> usize { ((WIDTH * c.1 + c.0) * 4) as usize }

#[derive(Clone, Debug)]
pub enum GraphicObjects {

    Pixel(DiCoordinate, Colour),

    Line(DiCoordinate, DiCoordinate, Colour),

    Triangle(DiCoordinate, DiCoordinate, DiCoordinate, Surface),


}
impl Rend for GraphicObjects {

    fn rend(&self, rendered: &mut Visual) {

        match self {

            GraphicObjects::Pixel(coordinate, colour) => {

                let index:usize = indexify(coordinate);

                rendered[index]     = colour.0;
                rendered[index + 1] = colour.1;
                rendered[index + 2] = colour.2;
                rendered[index + 3] = colour.3;

            },

            GraphicObjects::Line(coordinate1, coordinate2, colour) => {
                let mut x1 = coordinate1.0;
                let mut y1 = coordinate1.1;
                let mut x2 = coordinate2.0;
                let mut y2 = coordinate2.1;

                let dx = (x2 - x1).abs();
                let dy = (y2 - y1).abs();

                let sx = if x1 < x2 { 1 } else { -1 };
                let sy = if y1 < y2 { 1 } else { -1 };

                let mut err = dx - dy;

                loop {
                    let index: usize = indexify(&(x1, y1));

                    rendered[index] = colour.0;
                    rendered[index + 1] = colour.1;
                    rendered[index + 2] = colour.2;
                    rendered[index + 3] = colour.3;

                    if x1 == x2 && y1 == y2 {
                        break;
                    }

                    let e2 = 2 * err;

                    if e2 > -dy {
                        err -= dy;
                        x1 += sx;
                    }

                    if e2 < dx {
                        err += dx;
                        y1 += sy;
                    }
                }
            },


            GraphicObjects::Triangle(coordinate1, coordinate2, coordinate3, surface) => {
                let mut coords = vec![*coordinate1, *coordinate2, *coordinate3];

                coords.sort_by(|a, b| (a.0).cmp(&b.0));

                let coordinates = (*coords.get(0).unwrap(), *coords.get(1).unwrap(), *coords.get(2).unwrap());
                if coordinates.0.0 == coords.get(1).unwrap().0 {
                    let lines = vec![
                        line_between_points(coordinates.0, coordinates.2),
                        line_between_points(coordinates.1, coordinates.2)
                    ].into_iter().filter_map(|x| x).collect::<Vec<InfLine>>();


                    for x in 0..coordinates.2.0 - coordinates.0.0 {
                        let y1 = crossing_point(x, lines.get(0).copied()).unwrap();
                        let y2 = crossing_point(x, lines.get(1).copied()).unwrap();


                        for y in min(y1, y2)..max(y1, y2) {
                            let index: usize = indexify(&(x + coordinates.0.0, y));

                            match surface {
                                Flat(colour) => {
                                    rendered[index] = colour.0;
                                    rendered[index + 1] = colour.1;
                                    rendered[index + 2] = colour.2;
                                    rendered[index + 3] = colour.3;
                                }
                            }
                        }
                    }
                } else if coordinates.1.0 == coordinates.2.0 {
                    let lines = vec![
                        line_between_points(coordinates.0, coordinates.1),
                        line_between_points(coordinates.0, coordinates.2)
                    ].into_iter().filter_map(|x| x).collect::<Vec<InfLine>>();

                    for x in 0..coordinates.1.0 - coordinates.0.0 {
                        let y1 = crossing_point(x, lines.get(0).copied()).unwrap();
                        let y2 = crossing_point(x, lines.get(1).copied()).unwrap();

                        for y in min(y1, y2)..max(y1, y2) {
                            let index: usize = indexify(&(x + coordinates.0.0, y));

                            match surface {
                                Flat(colour) => {
                                    rendered[index] = colour.0;
                                    rendered[index + 1] = colour.1;
                                    rendered[index + 2] = colour.2;
                                    rendered[index + 3] = colour.3;
                                }
                            }
                        }
                    }
                } else {

                    panic!("use the complex triangle, dont fuck with graphic triangles")

                }
            }
        }

    }

}


#[derive(Clone, Debug, Copy)]
pub enum Surface {

    Flat(Colour)

}

type InfLine = (f32, i32);

fn crossing_point(x:i32, light: Option<InfLine>) -> Option<i32> {

    match light {
        Some((slope, intercept)) => unsafe {
            let y = slope * (x as f32) + (intercept as f32);
            Some(floorf32(y) as i32)
        },
        None => None
    }

    }


fn line_between_points(p1: DiCoordinate, p2: DiCoordinate) -> Option<InfLine> {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;

    if dx == 0 {
        return None;
    }

    let slope = dy as f32 / dx as f32;

    let intercept = p1.1;  //(p1.1 as f32 - (slope * p1.0 as f32)) as i32;

    Some((slope, intercept))
}


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

pub trait Transformation<Pivot> {

    fn rotate(&mut self, trans: Transformer, pivot: Pivot);

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

                    let line =  line_between_points(coordinates.0, coordinates.2).unwrap();

                    let next0 = (coordinates.1.0, crossing_point(coordinates.1.0 - coordinates.0.0, Some(line)).unwrap());

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




