
use crate::{HEIGHT, EVENTLOOP, WIDTH};
use crate::objects::{Objects, Rend, Visual};
use std::thread;
use std::time::Duration;


pub(crate) fn draw_gradient(pixels: &mut Vec<u8>, objects: Vec<Objects>) {

    thread::sleep(Duration::from_millis(20)); //144 fps

    unsafe {

        if objects.len() == 0 {
            //thread::yield_now();
            return
        }

        for object in objects.iter() {

            object.rend(pixels);


        }



    }


}


//dont make it reset every goddamn time
//object stack
pub trait GraphicProcess {

    fn spec(&mut self) -> Option<State>;
}

pub struct State {

    pub(crate) objects:Vec<Objects>,

    pub canvas:Option<Visual>,


}

