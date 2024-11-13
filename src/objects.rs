use std::cmp::{max, min};
use std::intrinsics::{floorf32, roundf32};
use std::process::exit;
use crate::{STATE, State, WIDTH, HEIGHT};
use crate::objects::Objects::Triangle;
use crate::objects::Surface::Flat;

pub type Visual = Vec<u8>;

pub trait Rend {
    fn rend(&self, rendered: &mut Visual, state: &State);
}


type Colour = (u8, u8, u8, u8);
type Coordinate = (i32, i32);


fn indexify(c:&Coordinate) -> usize { ((WIDTH * c.1 + c.0) * 4) as usize }

#[derive(Clone, Debug)]
pub enum Surface {

    Flat(Colour)

}

type InfLine = (f32, i32);

fn crossing_point(x:i32, light: Option<InfLine>) -> Option<i32> {

    match light {
        Some((slope, intercept)) => unsafe {
            let y = slope * (x as f32) + (intercept as f32);
            Some(roundf32(y) as i32)
        },
        None => None
    }

    }


fn line_between_points(p1: Coordinate, p2: Coordinate) -> Option<InfLine> {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;

    if dx == 0 {
        return None;
    }

    let slope = dy as f32 / dx as f32;

    let intercept = p1.1;  //(p1.1 as f32 - (slope * p1.0 as f32)) as i32;

    Some((slope, intercept))
}


#[derive(Clone, Debug)]
pub enum Objects {

    Point(Coordinate, Colour),

    Line(Coordinate, Coordinate, Colour),

    Triangle(Coordinate, Coordinate, Coordinate, Surface),

    Quadrangle(Coordinate, Coordinate, Coordinate, Coordinate, Surface)

}

impl Rend for Objects {
    fn rend(&self, rendered: &mut Visual, state: &State) {
        match self {
            Objects::Point(coordinate, colour) => {

                let index:usize = indexify(coordinate);

                rendered[index]     = colour.0;
                rendered[index + 1] = colour.1;
                rendered[index + 2] = colour.2;
                rendered[index + 3] = colour.3;

            },

            Objects::Line(coordinate1, coordinate2, colour) => {
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


            Objects::Triangle(coordinate1, coordinate2, coordinate3, surface) => {

                let mut coords = vec![*coordinate1, *coordinate2, *coordinate3];

                coords.sort_by(|a, b| (a.0).cmp(&b.0));

                if coords.get(0).unwrap().0 == coords.get(1).unwrap().0 {

                    let lines =  vec![
                        line_between_points(*coords.get(0).unwrap(), *coords.get(2).unwrap()),
                        line_between_points(*coords.get(1).unwrap(), *coords.get(2).unwrap())
                    ].into_iter().filter_map(|x| x).collect::<Vec<InfLine>>();

                    println!("{:?}", lines);

                    for x in 0 .. coords.get(2).unwrap().0 - coords.get(0).unwrap().0 {

                        let y1 = crossing_point(x, lines.get(0).copied()).unwrap();
                        let y2 = crossing_point(x, lines.get(1).copied()).unwrap();

                        println!("{} {} {}", x, y1, y2);

                        for y in min(y1, y2) .. max(y1, y2) {
                            let index:usize = indexify(&(x + coords.get(0).unwrap().0, y));

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

                } else if coords.get(1).unwrap().0 == coords.get(2).unwrap().0 {

                    let lines =  vec![
                        line_between_points(*coords.get(0).unwrap(), *coords.get(1).unwrap()),
                        line_between_points(*coords.get(0).unwrap(), *coords.get(2).unwrap())
                    ].into_iter().filter_map(|x| x).collect::<Vec<InfLine>>();

                    for x in 0 .. coords.get(1).unwrap().0 - coords.get(0).unwrap().0 {
                        let y1 = crossing_point(x, lines.get(0).copied()).unwrap();
                        let y2 = crossing_point(x, lines.get(1).copied()).unwrap();

                        for y in min(y1, y2)..max(y1, y2) {
                            let index: usize = indexify(&(x + coords.get(0).unwrap().0, y));

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

                    let line =  line_between_points(*coords.get(0).unwrap(), *coords.get(2).unwrap()).unwrap();

                    let next0 = (coords.get(1).unwrap().0, crossing_point(coords.get(1).unwrap().0, Some(line)).unwrap());

                    let triangles = (
                        Triangle(
                            *coords.get(0).unwrap(),
                            *coords.get(1).unwrap(),
                            next0,
                            (*surface).clone()
                        ),
                        Triangle(
                            *coords.get(2).unwrap(),
                            *coords.get(1).unwrap(),
                            next0,
                            (*surface).clone()
                        )
                    );


                    triangles.0.rend(rendered, state);
                    triangles.1.rend(rendered, state); // watch out opengl

                }

            },


            Objects::Quadrangle(coordinate1, coordinate2, coordinate3, coordinate4, surface) => {

                let mut sorted_by_x:Vec<Coordinate> = vec![*coordinate1, *coordinate2, *coordinate3, *coordinate4];

                sorted_by_x.sort_by(|a, b| (a.0).cmp(&b.0));

                let mut grouping = (
                    vec![*sorted_by_x.get(0).unwrap(), *sorted_by_x.get(1).unwrap()],
                    vec![*sorted_by_x.get(2).unwrap(), *sorted_by_x.get(3).unwrap()]

                );

                grouping.0.sort_by(|a, b| (a.1).cmp(&b.1));
                grouping.1.sort_by(|a, b| (a.1).cmp(&b.1));

                let triangles = (
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
                );

                triangles.0.rend(rendered, state);
                triangles.1.rend(rendered, state);

            }
        }
    }
}




