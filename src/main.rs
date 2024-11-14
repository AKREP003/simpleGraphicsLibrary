#![feature(core_intrinsics)]
mod WINdisplay;
mod render;
mod objects;
use std::collections::LinkedList;
use std::f32::consts::PI;
use objects::Rend;
use WINdisplay::{HEIGHT, run_window, WIDTH};
use render::{draw_gradient, State, GraphicProcess};
use crate::objects::{Objects, Visual};
use crate::objects::Objects::{Line, Point, Polygon, Quadrangle, Triangle};
use crate::objects::Surface::Flat;

static mut STATE:State = State {
    objects:vec![],
    canvas:Some(vec![])
};


static mut EVENTLOOP: SimpleOctagon= SimpleOctagon{init:false};

#[derive(Clone)]
struct SimpleOctagon {init:bool}

impl GraphicProcess for SimpleOctagon{

    fn spec(&mut self) -> Option<State> {

        if self.init {
            return None;
        }

        self.init  = true;

        Some(State {
            objects:vec![
                Polygon(4, 100, (200, 200), Flat((100, 200, 0, 0)))

            ],
            canvas:Some(vec![0u8; (WIDTH * HEIGHT * 4) as usize])
        })



    }

    
}

fn write_to_event_loop(loopy: SimpleOctagon) {
    unsafe {
        EVENTLOOP = loopy;
    }
}

fn main() {


    unsafe {



        run_window(draw_gradient);

    }
}