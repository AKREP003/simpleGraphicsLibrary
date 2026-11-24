#![feature(core_intrinsics)]
#![feature(let_chains)]
#![allow(static_mut_refs)]

extern crate core;


use std::collections::LinkedList;
use std::f32::consts::PI;
use std::intrinsics::{powf, sqrt};
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

use winapi::shared::minwindef::PBYTE;
use winapi::shared::windef::POINT;
use winapi::um::winuser::{GetCursorPos, GetKeyboardState};

use DiComplex::ComplexObjects;
use DiComplex::ComplexObjects::{CTriangle, Polygon, Qangle};
use graphics::Rend;
use render::{draw_gradient, Environment, State};
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
use crate::d3Viewer::render_scene;

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
mod d3Viewer;


static mut init: bool = true;

static degree: f64 = 30.0;
//todo https://en.wikipedia.org/wiki/Wavefront_.obj_file




fn main() {




    render_scene("C:/Users/aliek/Downloads/Exquisite Tumelo/tinker.obj".to_string());



}