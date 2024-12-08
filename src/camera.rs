use crate::graphics::DiCoordinate;
use crate::transitions::{from_angles, matrix_add, matrix_mult, matrix_sub, rotate_coordinate, Transformer, tri_to_di};
use crate::TriGraphics::{CartesianCoordinate, Oriantation};
use crate::WINdisplay::{HEIGHT, WIDTH};

pub struct Camera {

    pub position : CartesianCoordinate,
    pub orientation : Oriantation

}

impl Camera {

    pub fn projection(&self, c: CartesianCoordinate) -> CartesianCoordinate {
        let relative_position = matrix_sub(c, self.position);
        let rotation_matrix = from_angles(
            -self.orientation.0,
            -self.orientation.1,
            -self.orientation.2,
        );


        let b = matrix_add(matrix_mult(relative_position, rotation_matrix), (c.2, c.1, 0.0));

        println!("b: {:?}", b);

        return b;
    }

    pub fn project_to_2d(&self, c: CartesianCoordinate, focal_length: f64) -> (f64, f64) {
        let transformed = self.projection(c);

        (transformed.0, transformed.1)

        //perspective_projection(transformed, focal_length)
    }

}

pub fn perspective_projection(point: CartesianCoordinate, focal_length: f64) -> (f64, f64) {
    let (x, y, z) = point;
    if z != 0.0 {
        (x / z * focal_length, y / z * focal_length)
    } else {
        (x * focal_length, y * focal_length)
    }
}
