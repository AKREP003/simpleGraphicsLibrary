use crate::graphics::{Compile, GraphicObjects, Surface};
use crate::render::Arche;
use crate::render::Arche::TriC;
use crate::transitions::{rotate_coordinate, Transformation, Transformer};
use crate::TriComplex::TriComplexes::RectangularPrism;
use crate::TriGraphics::{CartesianCoordinate, TriQuadrangle};
use crate::TriGraphics::TriObjects::*;

//todo find center
//todo add templates
//todo optimize every function that doesnt take ref


#[derive(Clone, Debug, Copy)]
pub struct Rectprism {

    sides : [TriQuadrangle; 6],

    surfaces : [Surface; 6]

}
impl Rectprism {

    pub fn construct(
        pivot: CartesianCoordinate,
        dimensions: [i32; 3],
        surfaces: [Surface; 6],
    ) -> Rectprism {
        let mut corners: [CartesianCoordinate; 8] = [
            pivot.clone(),
            (pivot.0 + dimensions[0], pivot.1, pivot.2),
            (pivot.0, pivot.1 + dimensions[1], pivot.2),
            (pivot.0 + dimensions[0], pivot.1 + dimensions[1], pivot.2),
            (pivot.0, pivot.1, pivot.2 + dimensions[2]),
            (pivot.0 + dimensions[0], pivot.1, pivot.2 + dimensions[2]),
            (pivot.0, pivot.1 + dimensions[1], pivot.2 + dimensions[2]),
            (
                pivot.0 + dimensions[0],
                pivot.1 + dimensions[1],
                pivot.2 + dimensions[2],
            ),
        ];

        // Initialize sides array with dummy values
        let mut sides: [TriQuadrangle; 6] = [
            TriQuadrangle::construct(
                &mut [
                    corners[0],
                    corners[1],
                    corners[4],
                    corners[5],
                ],
                surfaces[0],
            ),
            TriQuadrangle::construct(
                &mut [
                    corners[1],
                    corners[3],
                    corners[5],
                    corners[7],
                ],
                surfaces[1],
            ),
            TriQuadrangle::construct(
                &mut [
                    corners[3],
                    corners[2],
                    corners[7],
                    corners[6],
                ],
                surfaces[2],
            ),
            TriQuadrangle::construct(
                &mut [
                    corners[2],
                    corners[0],
                    corners[6],
                    corners[4],
                ],
                surfaces[3],
            ),
            TriQuadrangle::construct(
                &mut [
                    corners[0],
                    corners[1],
                    corners[2],
                    corners[3],
                ],
                surfaces[4],
            ),
            TriQuadrangle::construct(
                &mut [
                    corners[4],
                    corners[5],
                    corners[6],
                    corners[7],
                ],
                surfaces[5],
            ),
        ];


        // Sort sides by their z-center values
        sides.sort_by(|s1, s2| s2.get_center().2.cmp(&s1.get_center().2));


        Rectprism { sides, surfaces }
    }




}

impl Compile for Rectprism {
    fn compile(&self) -> Vec<GraphicObjects> {

        let mut buffer = vec![];

        for side in self.sides {
            buffer.append(&mut side.compile());
        }

        return buffer
    }
}

impl Transformation<CartesianCoordinate> for Rectprism {
    fn rotate(&mut self, trans: Transformer, pivot: CartesianCoordinate) {


        for i in 0..6 {

            self.sides[i].rotate(trans, pivot);

        }

        self.sides.sort_by(|s1, s2| s2.get_center().2.cmp(&s1.get_center().2));

    }
}

#[derive(Clone, Debug, Copy)]
pub enum TriComplexes {

    RectangularPrism(
        Rectprism
    )

}



impl Compile for TriComplexes {
    fn compile(&self) -> Vec<GraphicObjects> {

        match self {
            TriComplexes::RectangularPrism(q) => {return q.compile()}
        }



    }

}

impl Transformation<CartesianCoordinate> for TriComplexes {
    fn rotate(&mut self, trans: Transformer, pivot: CartesianCoordinate) {

        match self {
            TriComplexes::RectangularPrism(q) => {
                q.rotate(trans, pivot)

            }

        }

    }
}

