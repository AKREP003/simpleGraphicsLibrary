#![feature(core_intrinsics)]
#![feature(let_chains)]

extern crate core;

use std::collections::LinkedList;
use std::f32::consts::PI;
use std::intrinsics::{powf64, sqrtf64};
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

use winapi::shared::minwindef::PBYTE;
use winapi::shared::windef::POINT;
use winapi::um::winuser::{GetCursorPos, GetKeyboardState};

use DiComplex::ComplexObjects;
use DiComplex::ComplexObjects::{CTriangle, Polygon, Qangle};
use graphics::Rend;
use render::{draw_gradient, State};
use transitions::Transformation;
use WINdisplay::{HEIGHT, run_window, WIDTH};

use crate::camera::Camera;
use crate::DiComplex::ComplexTriangle;
use crate::graphics::{Compile, Visual};
use crate::graphics::Surface::Flat;
use crate::ParseModel::compileOBJ;
use Arc::Arche::{Di, Tri, TriC};
use Arc::Arche;
use Arc::Arche::Null;
use crate::transitions::{from_angles, Transformer};
use crate::TriComplex::{Rectprism, TriComplexes};
use crate::TriComplex::TriComplexes::RectangularPrism;
use crate::TriGame::{cam, camera_transition};
use crate::TriGraphics::{TriObjects, TriQuadrangle, TriTriangle};
use crate::TriGraphics::TriObjects::{TriLine, TriTring};

mod WINdisplay;
mod render;
mod graphics;
mod TriGraphics;
mod DiComplex;
mod TriComplex;
mod transitions;
mod camera;
mod TriGame;
mod ParseModel;
mod Arc;

static mut init: bool = true;

static degree: f64 = 30.0;
//todo https://en.wikipedia.org/wiki/Wavefront_.obj_file


static mut SHAPE: Arche = Null; //ComplexTriangle::construct(&mut [(WIDTH / 2, HEIGHT / 2), ((WIDTH / 2) + 100, HEIGHT / 2), (WIDTH / 2, (HEIGHT / 2) + 100)], Flat((100, 200, 0, 0))).into();


//static mut TRILINE:TriObjects = TriLine((WIDTH / 2 + 100, (HEIGHT / 2) + 100, 1), (WIDTH / 2 - 100, (HEIGHT / 2) - 100, -1), (100, 200, 0, 0));
//static mut TRITRIANGLE:TriObjects = TriObjects::TriTriangle((WIDTH / 2, HEIGHT / 2, 0), ((WIDTH / 2) + 100, HEIGHT / 2, 1), (WIDTH / 2, (HEIGHT / 2) + 100, 0), Flat((100, 200, 0, 0)));
//static mut TRIQUADRANGLE:TriObjects = TriObjects::TriQuadrangle((WIDTH / 2, HEIGHT / 2, 1), ((WIDTH / 2) + 100, HEIGHT / 2, 1), (WIDTH / 2, (HEIGHT / 2) + 50, 1), (100 + (WIDTH / 2), (HEIGHT / 2) + 50, 1), Flat((100, 200, 0, 0)));


static mut LAST_RUN_TIME: Option<Instant> = None; // Static mutable variable to store the last run time
// todo: https://en.wikipedia.org/wiki/3D_projection


unsafe fn oct() -> Option<State> {
    let now = Instant::now();

    if let Some(last_time) = LAST_RUN_TIME && now.duration_since(last_time) < Duration::from_millis(40) {
        return None;
    } else { LAST_RUN_TIME = Some(now); }

    let r30: Transformer = from_angles(5.0, 5.0, 0.0);

    init = false;

    let mut piv = (0.0, 0.0, 0.0);

    let mut projection_buffer = Null;


    if let Arche::Tri(tri) = &SHAPE {
        if let TriObjects::TriTring(mut prism) = &tri {
            piv = prism.center;
            //SHAPE.rotate(r30, piv);
            prism.projection(&cam, 350.0);
            projection_buffer = prism.into();
        }

        if let TriObjects::TriQuad(mut prism) = &tri {
            piv = prism.center;
            SHAPE.rotate(r30, piv);
            prism.projection(&cam, 350.0);
            projection_buffer = prism.into();
        }
    };


    Some(State {
        objects: vec![
            projection_buffer
        ],
        canvas: Some(vec![0u8; (WIDTH * HEIGHT * 4) as usize]),
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
        SHAPE = compileOBJ(Box::from(Path::new("samples/simpleTriangle/triangle.obj"))).get(0).unwrap().clone().into();

        LAST_RUN_TIME = Some(Instant::now());

        run_window(draw_gradient, oct);
    }
}