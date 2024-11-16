#![feature(core_intrinsics)]
#![feature(let_chains)]

extern crate core;

mod WINdisplay;
mod render;
mod objects;
mod triD;

use std::collections::LinkedList;
use std::f32::consts::PI;
use objects::Rend;
use WINdisplay::{HEIGHT, run_window, WIDTH};
use render::{draw_gradient, State};
use crate::objects::{Compile, ComplexObjects, Transformation, Visual};
use crate::objects::ComplexObjects::{ Polygon, Quadrangle, ComplexTriangle};
use crate::objects::Surface::Flat;


static mut init:bool = true;

static r30: ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64)) = (
(0.866, -0.5, 0.0),
(0.5, 0.866, 0.0),
(0.0, 0.0, 1.0)
);

static mut SHAPE:ComplexObjects = ComplexTriangle((WIDTH / 2, HEIGHT / 2), ((WIDTH / 2) + 100, HEIGHT / 2), (WIDTH / 2, (HEIGHT / 2) + 100), Flat((100, 200, 0, 0)));

unsafe fn oct() -> Option<State> {

    init = false;

    SHAPE.transform(r30, (WIDTH / 2, HEIGHT / 2));

    Some(State {
        objects:vec![
            SHAPE.clone()
        ],
        canvas:Some(vec![0u8; (WIDTH * HEIGHT * 4) as usize])
    })



}

fn main() {


    unsafe {

        run_window(draw_gradient, oct);

    }
}