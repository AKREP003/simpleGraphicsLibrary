use crate::graphics::{Compile, GraphicObjects, Surface};
use crate::transitions::{rotate_coordinate, Transformation, Transformer};
use crate::TriGraphics::CartesianCoordinate;
use crate::TriGraphics::TriObjects::*;




#[derive(Clone, Debug, Copy)]
pub enum TriComplexes {

    RectangularPrism(
        CartesianCoordinate, CartesianCoordinate,
        CartesianCoordinate, CartesianCoordinate,
        CartesianCoordinate, CartesianCoordinate,
        CartesianCoordinate, CartesianCoordinate,
        Surface
    )

}


impl Compile for TriComplexes {
    fn compile(&self) -> Vec<GraphicObjects> {

        let mut grafic_buffer: Vec<GraphicObjects> = vec![];

        match self {
            TriComplexes::RectangularPrism(c1, c2,
                                           c3, c4,
                                           c5, c6,
                                           c7, c8,
                                           surface
            ) => {

                let mut sides
                    = [
                    vec![c1.clone()],
                    vec![],
                    vec![c1.clone()],
                    vec![],
                    vec![c1.clone()],
                    vec![],
                ];

                let mut corners = vec![*c2, *c3, *c4, *c5, *c6, *c7, *c8];

                for c@(x, y, z) in corners {

                    if c1.0 == x { sides[0].push(c); }
                    else { sides[1].push(c); }

                    if c1.1 == y { sides[2].push(c); }
                    else { sides[3].push(c); }

                    if c1.2 == z { sides[4].push(c); }
                    else { sides[5].push(c); }

                }


                 for side in sides {

                    grafic_buffer.append(
                        &mut TriQuadrangle(
                            side[0],
                            side[1],
                            side[2],
                            side[3],
                            surface.clone()
                        ).compile()

                    )

                 }

            }
        }


        return grafic_buffer;
    }

}

impl Transformation<CartesianCoordinate> for TriComplexes {
    fn rotate(&mut self, trans: Transformer, pivot: CartesianCoordinate) {

        match self {
            TriComplexes::RectangularPrism(c1, c2,
                                           c3, c4,
                                           c5, c6,
                                           c7, c8,
                                           _) => {

                *c1 = rotate_coordinate(*c1, trans, pivot);
                *c2 = rotate_coordinate(*c2, trans, pivot);
                *c3 = rotate_coordinate(*c3, trans, pivot);
                *c4 = rotate_coordinate(*c4, trans, pivot);
                *c5 = rotate_coordinate(*c5, trans, pivot);
                *c6 = rotate_coordinate(*c6, trans, pivot);
                *c7 = rotate_coordinate(*c7, trans, pivot);
                *c8 = rotate_coordinate(*c8, trans, pivot);

                println!("{:?}", self)

            }

        }

    }
}

