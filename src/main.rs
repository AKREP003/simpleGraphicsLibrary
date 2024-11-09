mod WINdisplay;
mod render;
mod objects;


use std::collections::LinkedList;
use objects::{Rend};

use WINdisplay::{run_window, HEIGHT, WIDTH};

use render::draw_gradient;
use crate::objects::{Objects};
use crate::objects::Objects::Point;


pub struct State {

    pub(crate) objects:Vec<Objects>

}

static mut STATE:State = State {
    objects:vec![]

};

fn main() {


    unsafe {
        STATE = State {
            objects: vec![Point(
                 (50, 50), (255, 0, 0, 0)
            )]
        };

    run_window(draw_gradient);

    }
}