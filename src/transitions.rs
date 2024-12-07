use crate::graphics::DiCoordinate;
use crate::TriGraphics::CartesianCoordinate;

pub(crate) type Transformer = ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64));

pub fn matrix_mult(
    vector: (i32, i32, i32),
    matrix: ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64))
) -> (i32, i32, i32) {

     (
        (vector.0 as f64 * matrix.0 .0 + vector.1 as f64 * matrix.1 .0 + vector.2 as f64 * matrix.2 .0).ceil() as i32,
        (vector.0 as f64 * matrix.0 .1 + vector.1 as f64 * matrix.1 .1 + vector.2 as f64 * matrix.2 .1).ceil() as i32,
        (vector.0 as f64* matrix.0 .2 + vector.1 as f64 * matrix.1 .2 + vector.2 as f64 * matrix.2 .2).ceil() as i32,
    )
}

pub fn di_to_tri((x, y): DiCoordinate) -> CartesianCoordinate { (x, y, 0) }

pub fn tri_to_di((x, y, _) : CartesianCoordinate) -> DiCoordinate {(x, y)}

pub fn matrix_sub((x1, y1, z1) : CartesianCoordinate, (x2, y2, z2) : CartesianCoordinate) -> CartesianCoordinate {(x1 - x2, y1 - y2, z1 - z2)}

pub fn matrix_add((x1, y1, z1) : CartesianCoordinate, (x2, y2, z2) : CartesianCoordinate) -> CartesianCoordinate {(x1 + x2, y1 + y2, z1 + z2)}

pub fn rotate_coordinate(x:CartesianCoordinate, t_matrix:Transformer, pivot:CartesianCoordinate) -> CartesianCoordinate {
    matrix_add(pivot, matrix_mult(matrix_sub(x, pivot.clone() ), t_matrix) )

}

pub trait Transformation<Pivot> {

    fn rotate(&mut self, trans: Transformer, pivot: Pivot);

}
