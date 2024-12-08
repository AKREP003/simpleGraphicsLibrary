#![feature(core_intrinsics)]
#![feature(let_chains)]

extern crate core;

mod WINdisplay;
mod render;
mod graphics;
mod TriGraphics;
mod DiComplex;
mod TriComplex;
mod transitions;
mod camera;

use std::collections::LinkedList;
use std::f32::consts::PI;
use std::time::{Duration, Instant};
use DiComplex::ComplexObjects;
use graphics::Rend;
use WINdisplay::{HEIGHT, run_window, WIDTH};
use render::{draw_gradient, State};
use crate::graphics::{Compile, Visual};
use DiComplex::ComplexObjects::{CTriangle, Polygon, Qangle};
use crate::DiComplex::ComplexTriangle;
use crate::graphics::Surface::Flat;
use crate::render::Arche::{Di, Tri, TriC};
use crate::TriComplex::{Rectprism, TriComplexes};
use crate::TriComplex::TriComplexes::RectangularPrism;
use crate::TriGraphics::{TriObjects, TriQuadrangle, TriTriangle};
use crate::TriGraphics::TriObjects::TriLine;
use lazy_static::lazy_static;
use transitions::Transformation;
use crate::camera::Camera;
use crate::render::Arche::Null;
use crate::render::Arche;
use crate::transitions::{from_angles, Transformer};
use mouse_position::mouse_position::{Mouse};


static mut init:bool = true;

static degree:f64 = 30.0;



static mut SHAPE:Arche = Null; //ComplexTriangle::construct(&mut [(WIDTH / 2, HEIGHT / 2), ((WIDTH / 2) + 100, HEIGHT / 2), (WIDTH / 2, (HEIGHT / 2) + 100)], Flat((100, 200, 0, 0))).into();


//static mut TRILINE:TriObjects = TriLine((WIDTH / 2 + 100, (HEIGHT / 2) + 100, 1), (WIDTH / 2 - 100, (HEIGHT / 2) - 100, -1), (100, 200, 0, 0));
//static mut TRITRIANGLE:TriObjects = TriObjects::TriTriangle((WIDTH / 2, HEIGHT / 2, 0), ((WIDTH / 2) + 100, HEIGHT / 2, 1), (WIDTH / 2, (HEIGHT / 2) + 100, 0), Flat((100, 200, 0, 0)));
//static mut TRIQUADRANGLE:TriObjects = TriObjects::TriQuadrangle((WIDTH / 2, HEIGHT / 2, 1), ((WIDTH / 2) + 100, HEIGHT / 2, 1), (WIDTH / 2, (HEIGHT / 2) + 50, 1), (100 + (WIDTH / 2), (HEIGHT / 2) + 50, 1), Flat((100, 200, 0, 0)));


static mut LAST_RUN_TIME: Option<Instant> = None; // Static mutable variable to store the last run time
// todo: https://en.wikipedia.org/wiki/3D_projection

static mut cam : Camera = Camera { position: (((WIDTH / 2) ) as f64, ((HEIGHT / 2) ) as f64, 0.0), orientation: (0.0, 0.0, 10.0) };

unsafe fn oct() -> Option<State> {

    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { x, y } => {

            cam.orientation.0 = ((((y as f64) - 540.0) / 1080.0) * 720.0);
            cam.orientation.1 = ((((x as f64) - 960.0) / 1919.0) * 360.0);

        },
        Mouse::Error => println!("Error getting mouse position"),
    }

    let now = Instant::now();

    if let Some(last_time) = LAST_RUN_TIME && now.duration_since(last_time) < Duration::from_millis(40) {
        return None
    } else { LAST_RUN_TIME = Some(now); }

    let r30: Transformer = from_angles(5.0, 5.0,0.0);

    init = false;

    let mut piv = (0.0,0.0,0.0);

    let mut projection_buffer = Null;



    if let Arche::TriC(tri) = &SHAPE { if let TriComplexes::RectangularPrism(prism) = &tri {
        piv = prism.center;
        SHAPE.rotate(r30, piv);
        projection_buffer = prism.projection(&cam, 100.0).into()
    }};



    Some(State {
        objects:vec![
            projection_buffer
        ],
        canvas:Some(vec![0u8; (WIDTH * HEIGHT * 4) as usize])
    })



}

fn main() {

    let colors = [
        Flat((100, 200, 0, 255)),
        Flat((0, 200, 0, 255)),
             Flat((0, 0, 200, 100)),
                  Flat((0, 100, 200, 255)),
                       Flat((0, 0, 0, 100)),
                            Flat((100, 100, 100, 0)),
    ];

    unsafe {
        SHAPE = Rectprism::construct((((WIDTH / 2) ) as f64, ((HEIGHT / 2) ) as f64, 100.0), [100.0, 50.0, 100.0],colors).into();

        LAST_RUN_TIME = Some(Instant::now());

        run_window(draw_gradient, oct);

    }
}