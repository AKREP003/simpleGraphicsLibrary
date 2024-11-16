#![feature(core_intrinsics)]
#![feature(let_chains)]

mod WINdisplay;
mod render;
mod objects;
mod triD;

use std::collections::LinkedList;
use std::f32::consts::PI;
use objects::Rend;
use WINdisplay::{HEIGHT, run_window, WIDTH};
use render::{draw_gradient, State};
use crate::objects::{ComplexObjects, Visual};
use crate::objects::ComplexObjects::{ Polygon, Quadrangle, ComplexTriangle};
use crate::objects::Surface::Flat;


static init:bool = true;

fn oct() -> Option<State> {

    if !&init {
        return None
    }

    Some(State {
        objects:vec![
            Polygon(4, 100, (200, 200), Flat((100, 200, 0, 0)))

        ],
        canvas:None
    })



}

fn main() {


    unsafe {

        run_window(draw_gradient, oct);

    }
}