use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path;
use std::path::Path;
use crate::TriGraphics::TriObjects;
use crate::graphics::Colour;
use crate::DiComplex::Quadrangle;
use crate::graphics::Surface;
use crate::graphics::Surface::Flat;
use crate::Arc::Arche;
use crate::TriGraphics::{CartesianCoordinate, TriQuadrangle, TriTriangle};

static SIZE_COEFFICIENT: f64 = 100.0;

fn parse_vert(vert: &str) -> (usize, Option<usize>) {

    let split = vert.split("/").collect::<Vec<&str>>();

    let v = split[0].parse::<usize>().unwrap();

    let vt = if split.len() == 2 { Some(split[1].parse::<usize>().unwrap()) } else { None };

    (v, vt)

}

#[feature(core_io_internals)]
fn parseMTL(path : Box<Path>, target: &mut HashMap<String, Surface>){

    let mut file = File::open(path).unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let lines = contents.lines();

    let mut current_material:Option<String> = None;

    let mut current_surface:Option<Surface> = None;

    for line in lines {

        let elements:Vec<&str> = line.split(" ").collect();

        if  elements.len() == 0 {continue}

        match elements[0] {

            "newmtl" => {

                if elements.len() != 2 {continue;}

                if current_material != None {

                    target.insert(current_material.unwrap(), current_surface.unwrap());

                }

                current_material = Some(elements[1].to_owned());

            }

            "Kd" => {

                if elements.len() != 4 {continue};

                let d:u8 = if let Some(Flat((_, _,_, n))) = current_surface {n} else { 255 };

                let mut surface_color: Colour = (0,0,0,d);

                let color_normal = |i:usize| {(elements[i].parse().unwrap_or(0.0) * 255.0) as u8};

                surface_color.0 = color_normal(1);
                surface_color.1 = color_normal(2);
                surface_color.2 = color_normal(3);

                current_surface = Some(Flat(surface_color));
            }

            "d" => {

                if elements.len() != 2 {continue};

                let d = elements[1].parse().unwrap_or(1.0);

                let mut surface_color: Colour = (0, 0, 0, 0);

                if let Some(Flat((r, b,g, _))) = current_surface { surface_color = (r, b, g, 0) };

                surface_color.3 = (d * 255.0) as u8;

                current_surface = Some(Flat(surface_color));
            }

            _ => {}
        }

    }

    if current_material != None {
        target.insert(current_material.unwrap(), current_surface.unwrap());
    }

}


//https://paulbourke.net/dataformats/mtl/
//https://en.wikipedia.org/wiki/Wavefront_.obj_file

pub fn compileOBJ (path : Box<Path>) -> Vec<Arche>{

    let mut file = File::open(path.clone()).unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let lines = contents.lines();

    let mut vertices:Vec<CartesianCoordinate> = vec![];

    let mut objects:Vec<Arche> = vec![];

    let mut materials:HashMap<String, Surface> = HashMap::new();

    let mut current_surface:Surface =  Flat((100, 200, 0, 255));

    for line in lines {

        let elements:Vec<&str> = line.split(" ").collect();

        if  elements.len() == 0 {continue}

        match elements[0] {

            "#" => {}

            "mtllib" => {

                if elements.len() != 2 {continue;}

                let mtl_path = path.parent().expect("REASON").join(elements[1]);

                parseMTL(Box::from(mtl_path), &mut materials);

            }

            "usemtl" => {

                if elements.len() != 2 {continue;}

                let material = elements[1];

                let surface = materials.get(material).unwrap();

                current_surface = surface.clone();


            }

            "v" => {

                let x = elements[1].parse::<f64>().unwrap();
                let y = elements[2].parse::<f64>().unwrap();
                let z = elements[3].parse::<f64>().unwrap();

                vertices.push((x , y , z ));

            }

            "vt" => {}

            "vn" => {}

            "f" => {

                let params : Vec<(usize, Option<usize>)> = elements[1..].iter().map(|x| parse_vert(x)).collect();

                let mut points:Vec<CartesianCoordinate> = params.iter().map(|x| vertices[x.0 - 1]).collect();

                if elements.len() == 4 {

                    objects.push(

                        TriTriangle::construct(&mut [points.remove(0), points.remove(0), points.remove(0)], current_surface).into()

                    )

                }

                if elements.len() == 5 {

                    objects.push(

                        TriQuadrangle::construct(&mut [points.remove(0), points.remove(0), points.remove(0), points.remove(0)], current_surface).into()

                    )

                }

            }

            _ => {}
        }
    }

    objects

}

pub fn compileMTLLIB(path : Box<Path>) -> HashMap<String, Surface> {

    let mut buffer : HashMap<String, Surface> = HashMap::new();

    let mut file = File::open(path).unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let lines = contents.lines();
    for line in lines {
        let elements: Vec<&str> = line.split(" ").collect();

        if elements.len() == 0 { continue }

        match elements[0] {
            "#" => {}

            "newmtl" => {}

            "Ns" => {}

            "Ka" => {}

            "Kd" => {}

            "Ks" => {}

            "Ke" => {}

            "Ni" => {}

            "d" => {}

            "illum" => {}

            _ => {}
        }
    }

    buffer

}