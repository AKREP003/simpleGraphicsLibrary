use crate::{STATE, State, WIDTH, HEIGHT};
pub type Visual = Vec<u8>;

pub trait Rend {
    fn rend(&self, rendered: &mut Visual, state: &State);
}


type Colour = (u8, u8, u8, u8);
type Coordinate = (i32, i32);


fn indexify(c:&Coordinate) -> usize { ((WIDTH * c.1 + c.0) * 4) as usize }

pub enum Surface {

    Flat(Colour)

}

pub enum Objects {

    Point(Coordinate, Colour),

    Line(Coordinate, Coordinate, Colour)

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
            }
        }
    }
}




