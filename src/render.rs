
use crate::{HEIGHT, STATE, WIDTH};
use crate::objects::{Objects, Rend, Visual};
use std::thread;
use std::time::Duration;


pub(crate) fn draw_gradient(pixels: &mut Vec<u8>) {

    thread::sleep(Duration::from_millis(20)); //144 fps

    unsafe {

        if STATE.objects.len() == 0 {
            //thread::yield_now();
            return
        }

        for object in STATE.objects.iter() {



            object.rend(pixels, &STATE)

        }

        STATE.objects=vec![];



    }


}


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

