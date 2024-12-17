use crate::{HEIGHT, WIDTH};
use crate::Arc::Arche;
use crate::Arc::Arche::Tri;
use crate::DiComplex::{ComplexObjects, ComplexTriangle, Quadrangle};
use crate::DiComplex::ComplexObjects::Polygon;
use crate::graphics::{GraphicObjects, GraphicTriangle, Rend, Visual};
use crate::graphics::Compile;
use crate::graphics::Surface::Flat;
use crate::transitions::{Transformation, Transformer};
use crate::TriComplex::{Rectprism, TriComplexes};
use crate::TriGraphics::{CartesianCoordinate, TriObjects, TriQuadrangle, TriTriangle};
use crate::TriGraphics::TriObjects::TriTring;

#[link(name = "kernel")] // Links to `kernel.dll` (omit the extension)
extern "C" {
    //fn addWithCuda(c: *mut i32, a: *const i32, b: *const i32, n: i32);
    fn iterateOver(data: *const MyStruct, size: usize, struct_size: usize);
}

#[repr(C)]
struct MyStruct {
    x: i32
}

pub(crate) fn draw_gradient(pixels: &mut Vec<u8>, objects: Vec<Arche>) {
    unsafe {

        if objects.len() == 0 {
            //thread::yield_now();
            return;
        }

        let mut triangle_buffer: Vec<GraphicTriangle> = vec![];

        for object in objects.iter() {
            for graphic in &mut object.compile() {

                match graphic {
                    GraphicObjects::Pixel(_, _) => {graphic.rend(pixels);}
                    GraphicObjects::Line(_, _, _) => {graphic.rend(pixels);}
                    GraphicObjects::Triangle(triangle) => {
                        triangle_buffer.push(*triangle);
                    }
                }

            };
        }

        let mut vec: Vec<MyStruct> = vec![MyStruct {x:1}, MyStruct {x:2}, MyStruct {x:3}, MyStruct {x:4}, MyStruct {x:5}];

        iterateOver(
            vec.as_ptr(),
            vec.len(),
            std::mem::size_of::<MyStruct>(),
        );
    }
}

pub struct State {
    pub(crate) objects: Vec<Arche>,

    pub canvas: Option<Visual>,

}

