use std::thread;
use crate::{HEIGHT, WIDTH};
use crate::Arc::Arche;
use crate::Arc::Arche::Tri;
use crate::DiComplex::{ComplexObjects, ComplexTriangle, Quadrangle};
use crate::DiComplex::ComplexObjects::Polygon;
use crate::graphics::{Colour, DiCoordinate, GraphicObjects, GraphicTriangle, InfLine, Rend, Visual};
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
    fn drawTriangles(canvas: *const u8, width:i32, height : i32, data : *const GraphicTriangleC ,size: usize);
}

#[repr(C)]
struct MyStruct {
    x: i32
}

#[derive(Clone, Debug, Copy)]
#[repr(C)]
struct GraphicTriangleC {
    lines: [InfLine; 2],

    pub coords: [i32; 2],

}

pub(crate) fn draw_gradient(pixels: &mut Vec<u8>, objects: Vec<Arche>) {
    unsafe {

        if objects.len() == 0 {
            //thread::yield_now();
            return;
        }

        let mut triangle_buffer: Vec<GraphicTriangleC> = vec![];

        for object in objects.iter() {
            for graphic in &mut object.compile() {

                if let GraphicObjects::Triangle(graphic) = graphic {
                    triangle_buffer.push(GraphicTriangleC {lines : graphic.lines, coords: graphic.coords});
                    continue
                }

                graphic.rend(pixels);

            };
        }

        let status = drawTriangles(pixels.as_mut_ptr(), WIDTH, HEIGHT, triangle_buffer.as_ptr(), triangle_buffer.len());


        //iterateOver(vec.as_ptr(), vec.len(), std::mem::size_of::<MyStruct>(), );
    }
}

pub struct State {
    pub(crate) objects: Vec<Arche>,

    pub canvas: Option<Visual>,

}

