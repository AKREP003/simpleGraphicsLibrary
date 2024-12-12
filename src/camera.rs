use crate::graphics::DiCoordinate;
use crate::transitions::{from_angles, matrix_add, matrix_mult, matrix_sub, rotate_coordinate, Transformer, tri_to_di};
use crate::TriGraphics::{CartesianCoordinate, Oriantation};
use crate::WINdisplay::{HEIGHT, WIDTH};

pub struct Camera {
    pub position: CartesianCoordinate,
    pub orientation: Oriantation,
}

impl Camera {
    pub fn projection(&self, c: CartesianCoordinate, focal_length: f64) -> CartesianCoordinate {
        let relative_position = matrix_sub(c, self.position);
        let rotation_matrix = from_angles(
            -self.orientation.0,
            -self.orientation.1,
            -self.orientation.2,
        );

        //println!("{:?}", relative_position);

        let (x, y, z) = matrix_mult(relative_position, rotation_matrix);
        if z <= 0.0 {
            return (((x) * focal_length) + (WIDTH as f64 / 2.0), ((y) * focal_length) + (HEIGHT as f64 / 2.0), z);
        }

        return (((x / z) * focal_length) + (WIDTH as f64 / 2.0), ((y / z) * focal_length) + (HEIGHT as f64 / 2.0), 0.0);
    }
}

