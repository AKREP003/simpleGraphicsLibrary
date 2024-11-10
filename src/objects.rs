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
                match surface {
                    Surface::Flat(colour) => {
                        let mut x1 = coordinate1.0;
                        let mut y1 = coordinate1.1;
                        let mut x2 = coordinate2.0;
                        let mut y2 = coordinate2.1;
                        let mut x3 = coordinate3.0;
                        let mut y3 = coordinate3.1;

                        let mut coordinates = vec![coordinate1, coordinate2, coordinate3];
                        coordinates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                        let mut y = coordinates[0].1;
                        let mut y_max = coordinates[2].1;

                        while y < y_max {
                            let mut x = coordinates[0].0 + ((y - coordinates[0].1) as f32 * (coordinates[2].0 - coordinates[0].0) as f32 / (coordinates[2].1 - coordinates[0].1) as f32) as i32;
                            let mut x_max = coordinates[1].0 + ((y - coordinates[1].1) as f32 * (coordinates[2].0 - coordinates[1].0) as f32 / (coordinates[2].1 - coordinates[1].1) as f32) as i32;

                            if x > x_max {
                                let temp = x;
                                x = x_max;
                                x_max = temp;
                            }

                            for x in x..x_max {
                                let index: usize = indexify(&(x, y));

                                rendered[index] = colour.0;
                                rendered[index + 1] = colour.1;
                                rendered[index + 2] = colour.2;
                                rendered[index + 3] = colour.3;
                            }

                            y += 1;
                        }
                    }
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

                println!("{:?}", triangles.0);
                println!("{:?}", triangles.1);

                triangles.0.rend(rendered, state);
                //triangles.1.rend(rendered, state);

            }
        }
    }
}




