mod WINdisplay;
mod render;
mod objects;


use std::collections::LinkedList;
use objects::{Pixel, Rend};

use WINdisplay::{run_window, HEIGHT, WIDTH};

use render::draw_gradient;



pub struct State {

    pub(crate) objects:Vec<Pixel>

}

static mut STATE:State = State {
    objects:vec![]

};

fn main() {


    unsafe {
        STATE = State {
            objects: vec![Pixel {
                r: 255,
                g: 0,
                b: 0,
                a: 0,
            }; (HEIGHT * WIDTH) as usize]
        };
    }

    run_window(draw_gradient);
}