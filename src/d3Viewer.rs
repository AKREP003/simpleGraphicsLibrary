use std::path::Path;
use crate::Arc::Arche;
use crate::ParseModel::compileOBJ;



pub fn render_scene(obj_file: String) {

    let objects:Vec<Arche> = compileOBJ(Box::from(Path::new(&obj_file)));

    println!("{:?}", objects);

}
