#![feature(core_intrinsics)]
mod WINdisplay;
mod render;
mod objects;
use std::collections::LinkedList;
use std::f32::consts::PI;
use objects::Rend;
use WINdisplay::{HEIGHT, run_window, WIDTH};
use render::{draw_gradient, State};
use crate::objects::{Objects, Visual};
use crate::objects::Objects::{Line, Point, Polygon, Quadrangle, Triangle};
use crate::objects::Surface::Flat;

static mut STATE:State = State {
    objects:vec![],
    canvas:vec![]
};

fn main() {


    unsafe {
        STATE.objects =  vec![


            //Triangle((131, 200), (131, 200), (130, 200), Flat((255, 0, 0, 0))),

            //Quadrangle((100, 50), (200, 250), (130, 200), (250, 100), Flat((100, 200, 0, 0))),

            Polygon(25, 100, (200, 200), Flat((100, 200, 0, 0)))

        ];
        STATE.canvas = vec![0u8; (WIDTH * HEIGHT * 4) as usize];


    run_window(draw_gradient);

    }
}