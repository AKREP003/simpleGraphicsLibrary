#![feature(core_intrinsics)]
mod WINdisplay;
mod render;
mod objects;


use std::collections::LinkedList;
use objects::{Rend};

use WINdisplay::{run_window, HEIGHT, WIDTH};

use render::draw_gradient;
use crate::objects::{Objects, Visual};
use crate::objects::Objects::{Line, Point, Quadrangle, Triangle};
use crate::objects::Surface::Flat;


//dont make it reset every goddamn time
//object stack
trait GraphicProcess {

    fn init() -> State;
    fn next(state:State) -> State;
}

pub struct State {

    pub(crate) objects:Vec<Objects>,

    pub canvas:Visual,

}

static mut STATE:State = State {
    objects:vec![],
    canvas:vec![]
};

fn main() {


    unsafe {
        STATE.objects =  vec![


            //Triangle((200, 200), (100, 200), (250, 100), Flat((255, 0, 0, 0))),

            Quadrangle((100, 100), (200, 250), (130, 200), (250, 100), Flat((100, 200, 0, 0)))

        ];
        STATE.canvas = vec![0u8; (WIDTH * HEIGHT * 4) as usize];

    run_window(draw_gradient);

    }
}