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

    Point(Coordinate, Colour)

}

impl Rend for Objects {
    fn rend(&self, rendered: &mut Visual, state: &State) {
        match self {
            Objects::Point(coordinate, colour) => {

                let index:usize = indexify(coordinate);

                println!("index: {}", index);

                rendered[index]     = colour.0;
                rendered[index + 1] = colour.1;
                rendered[index + 2] = colour.2;
                rendered[index + 3] = colour.3;

            }
        }
    }
}




