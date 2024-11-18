#![feature(core_intrinsics)]
#![feature(let_chains)]

extern crate core;

mod WINdisplay;
mod render;
mod graphics;
mod TriGraphics;
mod DiComplex;

use std::collections::LinkedList;
use std::f32::consts::PI;
use DiComplex::ComplexObjects;
use graphics::Rend;
use WINdisplay::{HEIGHT, run_window, WIDTH};
use render::{draw_gradient, State};
use crate::graphics::{Compile, Transformation, Visual};
use DiComplex::ComplexObjects::{ComplexTriangle, Polygon, Quadrangle};
use crate::graphics::Surface::Flat;
use crate::render::Arche::{Di, Tri};
use crate::TriGraphics::TriObjects;
use crate::TriGraphics::TriObjects::TriLine;


static mut init:bool = true;

static degree:f64 = 5.0;

static mut SHAPE:ComplexObjects = ComplexTriangle((WIDTH / 2, HEIGHT / 2), ((WIDTH / 2) + 100, HEIGHT / 2), (WIDTH / 2, (HEIGHT / 2) + 100), Flat((100, 200, 0, 0)));
static mut TRILINE:TriObjects = TriLine((WIDTH / 2 + 100, (HEIGHT / 2) + 100, 1), (WIDTH / 2 - 100, (HEIGHT / 2) - 100, -1), (100, 200, 0, 0));
static mut TRITRIANGLE:TriObjects = TriObjects::TriTriangle((WIDTH / 2, HEIGHT / 2, 0), ((WIDTH / 2) + 100, HEIGHT / 2, 1), (WIDTH / 2, (HEIGHT / 2) + 100, 0), Flat((100, 200, 0, 0)));

unsafe fn oct() -> Option<State> {

    let r30: ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64)) = (
        (degree.cos(), 0.0, degree.sin()),
        (0.0, 1.0, 0.0),
        (-degree.sin(), 0.0, degree.cos())
    );

    init = false;

    TRITRIANGLE.rotate(r30, (WIDTH / 2 - 100, (HEIGHT / 2) - 100, -1));

    Some(State {
        objects:vec![
            Tri(TRITRIANGLE.clone())
        ],
        canvas:Some(vec![0u8; (WIDTH * HEIGHT * 4) as usize])
    })



}

fn main() {


    unsafe {

        run_window(draw_gradient, oct);

    }
}