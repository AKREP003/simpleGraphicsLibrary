
pub type SphericalCoordinate = (f32, f32, f32);
pub type CartesianCoordinate = (i32, i32, i32);

type Transformation =  ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32));

fn spherical_to_cartesian(c: SphericalCoordinate) -> CartesianCoordinate {
    let (r, theta, phi) = c;
    let x = r * theta.cos() * phi.sin();
    let y = r * theta.sin() * phi.sin();
    let z = r * phi.cos();
    (x as i32, y as i32, z as i32)
}

pub enum TriObjects {
    Cube()
}