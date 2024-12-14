use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
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

//https://paulbourke.net/dataformats/mtl/
//https://en.wikipedia.org/wiki/Wavefront_.obj_file

pub fn compileOBJ (path : Box<Path>) -> Vec<Arche>{

    let mut file = File::open(path).unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let lines = contents.lines();

    let mut vertices:Vec<CartesianCoordinate> = vec![];

    let mut objects:Vec<Arche> = vec![];

    for line in lines {

        let elements:Vec<&str> = line.split(" ").collect();

        if  elements.len() == 0 {continue}

        match elements[0] {

            "#" => {}

            "mtllib" => {}

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

                        TriTriangle::construct(&mut [points.remove(0), points.remove(0), points.remove(0)], Flat((100, 200, 0, 255))).into()

                    )

                }

                if elements.len() == 5 {

                    objects.push(

                        TriQuadrangle::construct(&mut [points.remove(0), points.remove(0), points.remove(0), points.remove(0)], Flat((100, 200, 0, 255))).into()

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