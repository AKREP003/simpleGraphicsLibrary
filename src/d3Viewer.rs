use std::path::Path;
use crate::render::{draw_gradient, State};
use crate::WINdisplay::run_window;
use crate::render::Environment;
use crate::TriGraphics::CartesianCoordinate;
use crate::Arc::Arche;
use crate::ParseModel::compileOBJ;
use crate::transitions::Transformation;

pub fn vector_magnitude(v: CartesianCoordinate) -> f64 {
  return (v.0.powi(2) + v.1.powi(2) + v.2.powi(2)).sqrt();
}

pub fn order_objects(objects: &Vec<Arche>) -> Vec<Arche>{

    let mut binding:Vec<Arche> = objects.clone();

    binding.iter_mut().for_each(|mut o| o.project(600.0));

    //binding.retain(|o| vector_magnitude(o.get_cntr()) > 0.0);

    binding.sort_by(|a, b| vector_magnitude(a.get_cntr()).total_cmp(&vector_magnitude(b.get_cntr())));

    return binding;

}


struct SimpleScene {

    objects: Vec<Arche>

}

impl Environment for SimpleScene {
    unsafe fn next_state(&self) -> Option<State> {
        Some(State {
            objects: order_objects(&self.objects),
            canvas: None,
        })
    }
}

pub fn render_scene(obj_file: String) {

    let objects:Vec<Arche> = compileOBJ(Box::from(Path::new(&obj_file)));

    let scene = SimpleScene { objects };

    run_window(draw_gradient, Box::from(scene));

}
