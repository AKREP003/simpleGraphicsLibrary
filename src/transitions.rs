use std::f64::consts::PI;

use crate::graphics::DiCoordinate;
use crate::TriGraphics::CartesianCoordinate;

pub fn from_angles(x_angle: f64, y_angle: f64, z_angle: f64) -> Transformer {
    // Convert degrees to radians
    let (x_rad, y_rad, z_rad) = (
        x_angle * PI / 180.0,
        y_angle * PI / 180.0,
        z_angle * PI / 180.0,
    );

    // Precompute sines and cosines
    let (sin_x, cos_x) = (x_rad.sin(), x_rad.cos());
    let (sin_y, cos_y) = (y_rad.sin(), y_rad.cos());
    let (sin_z, cos_z) = (z_rad.sin(), z_rad.cos());

    // Compute combined rotation matrix
    let m00 = cos_y * cos_z;
    let m01 = cos_y * sin_z;
    let m02 = -sin_y;

    let m10 = sin_x * sin_y * cos_z - cos_x * sin_z;
    let m11 = sin_x * sin_y * sin_z + cos_x * cos_z;
    let m12 = sin_x * cos_y;

    let m20 = cos_x * sin_y * cos_z + sin_x * sin_z;
    let m21 = cos_x * sin_y * sin_z - sin_x * cos_z;
    let m22 = cos_x * cos_y;

    (
        (m00, m01, m02),
        (m10, m11, m12),
        (m20, m21, m22),
    )
}


pub(crate) type Transformer = ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64));

pub fn matrix_mult(
    vector: (f64, f64, f64),
    matrix: ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64)),
) -> (f64, f64, f64) {
    (
        (vector.0 as f64 * matrix.0.0 + vector.1 as f64 * matrix.1.0 + vector.2 as f64 * matrix.2.0),
        (vector.0 as f64 * matrix.0.1 + vector.1 as f64 * matrix.1.1 + vector.2 as f64 * matrix.2.1),
        (vector.0 as f64 * matrix.0.2 + vector.1 as f64 * matrix.1.2 + vector.2 as f64 * matrix.2.2),
    )
}

pub fn di_to_tri((x, y): DiCoordinate) -> CartesianCoordinate { (x as f64, y as f64, 0.0) }

pub fn tri_to_di((x, y, _): CartesianCoordinate) -> DiCoordinate { (x.round() as i32, y.round() as i32) }

pub fn matrix_sub((x1, y1, z1): CartesianCoordinate, (x2, y2, z2): CartesianCoordinate) -> CartesianCoordinate { (x1 - x2, y1 - y2, z1 - z2) }

pub fn matrix_add((x1, y1, z1): CartesianCoordinate, (x2, y2, z2): CartesianCoordinate) -> CartesianCoordinate { (x1 + x2, y1 + y2, z1 + z2) }

pub fn rotate_coordinate(x: CartesianCoordinate, t_matrix: Transformer, pivot: CartesianCoordinate) -> CartesianCoordinate {
    matrix_add(pivot, matrix_mult(matrix_sub(x, pivot.clone()), t_matrix))
}

pub trait Transformation<Pivot> {
    fn rotate(&mut self, trans: Transformer, pivot: Pivot);
}
