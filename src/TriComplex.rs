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

    pub fn construct(pivot : CartesianCoordinate, dimensions : [i32; 3], surfaces :  [Surface; 6]) -> Rectprism {

        let mut dummy = [(0,0,0), (1,1,1), (2,2,2), (3,3,3)];

        let mut sides : [TriQuadrangle; 6] = [TriQuadrangle::construct(&mut dummy, surfaces[0]); 6];

        let mut corners : [CartesianCoordinate; 8] = [
            pivot.clone(),
            (pivot.0 + dimensions[0], pivot.1, pivot.2),
            (pivot.0, pivot.1 + dimensions[1], pivot.2),
            (pivot.0 + dimensions[0], pivot.1 + dimensions[1], pivot.2),
            (pivot.0, pivot.1, pivot.2 + dimensions[2]),
            (pivot.0 + dimensions[0], pivot.1, pivot.2 + dimensions[2]),
            (pivot.0, pivot.1 + dimensions[1], pivot.2 + dimensions[2]),
            (pivot.0 + dimensions[0], pivot.1 + dimensions[1], pivot.2 + dimensions[2])
        ];


        for i in 0..6 {

            sides[i] = TriQuadrangle::construct(
                &mut [corners[i],
                corners[(i + 1) % 4],
                corners[(i + 2) % 4 + 4],
                corners[(i + 3) % 4 + 4]],
                surfaces[i]
            );

        }

        return Rectprism { sides, surfaces };

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

