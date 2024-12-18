use std::cmp::{max, min};
use std::f32::consts::PI;
use std::intrinsics::{ceilf32, floorf32, roundf32};
use std::process::exit;

use crate::{HEIGHT, WIDTH};
use crate::DiComplex::ComplexObjects::CTriangle;
use crate::graphics::GraphicObjects::Triangle;
use crate::graphics::Surface::Flat;
use crate::render::State;
use crate::transitions::{rotate_coordinate, Transformation, Transformer};
use crate::TriGraphics::CartesianCoordinate;

pub type Visual = Vec<u8>;

pub trait Rend {
    fn rend(&self, rendered: &mut Visual);
}

pub trait Compile {
    fn compile(&self) -> Vec<GraphicObjects>;
}

pub type Colour = (u8, u8, u8, u8);

pub type DiCoordinate = (i32, i32);

fn indexify(c: &DiCoordinate) -> usize { ((WIDTH * c.1 + c.0) * 4) as usize }

#[derive(Clone, Debug, Copy)]
pub struct GraphicTriangle {
    pub(crate) lines: [InfLine; 2],

    pub coords: [i32; 2],

    pub surf: Surface,
}

impl GraphicTriangle {
    pub fn construct(mut coords: [DiCoordinate; 3], surf: Surface) -> GraphicTriangle {
        coords.sort_by(|a, b| (a.0).cmp(&b.0));

        let mut lines: [InfLine; 2];

        let direction = coords[0].0 == coords[1].0;

        if direction {
            lines = [
                line_between_points(coords[0], coords[2]),
                line_between_points(coords[1], coords[2])
            ].into_iter().filter_map(|x| x).collect::<Vec<InfLine>>().try_into().expect("eee");
        } else if coords[1].0 == coords[2].0 {
            lines = [
                line_between_points(coords[0], coords[1]),
                line_between_points(coords[0], coords[2])
            ].into_iter().filter_map(|x| x).collect::<Vec<InfLine>>().try_into().expect("eee");
        } else {
            panic!("use the complex triangle, dont fuck with graphic triangles")
        }

        return GraphicTriangle { lines, coords : [coords[0].0, coords[2].0], surf };
    }

    pub fn into(&self) -> GraphicObjects { Triangle(*self) }
}

impl Rend for GraphicTriangle {
    fn rend(&self, rendered: &mut Visual) {

        for x in 0..self.coords[1] - self.coords[0] {

            let r_x = x + self.coords[0];

            if r_x < 2 || r_x > WIDTH { continue; }

            let y1 = crossing_point(x, self.lines.get(0).copied()).unwrap();
            let y2 = crossing_point(x, self.lines.get(1).copied()).unwrap();

            for y in min(y1, y2)..max(y1, y2) {
                let coord = (r_x, y);

                if y > 1 && y < HEIGHT - 2 {

                    paint_it(rendered, self.surf, &coord);
                }
            }
        }

    }
}

#[derive(Clone, Debug, Copy)]
pub enum GraphicObjects {
    Pixel(DiCoordinate, Colour),

    Line(DiCoordinate, DiCoordinate, Colour),

    Triangle(GraphicTriangle),

}

impl From<GraphicTriangle> for GraphicObjects {
    fn from(value: GraphicTriangle) -> Self {
        Triangle(value)
    }
}

impl Rend for GraphicObjects {
    fn rend(&self, rendered: &mut Visual) {
        match self {
            GraphicObjects::Pixel(coordinate, colour) => {
                let index: usize = indexify(coordinate);

                rendered[index] = colour.0;
                rendered[index + 1] = colour.1;
                rendered[index + 2] = colour.2;
                rendered[index + 3] = colour.3;
            }

            GraphicObjects::Line(coordinate1, coordinate2, colour) => {
                let mut x1 = coordinate1.0;
                let mut y1 = coordinate1.1;
                let mut x2 = coordinate2.0;
                let mut y2 = coordinate2.1;

                let dx = (x2 - x1).abs();
                let dy = (y2 - y1).abs();

                let sx = if x1 < x2 { 1 } else { -1 };
                let sy = if y1 < y2 { 1 } else { -1 };

                let mut err = dx - dy;

                loop {
                    let index: usize = indexify(&(x1, y1));

                    rendered[index] = colour.0;
                    rendered[index + 1] = colour.1;
                    rendered[index + 2] = colour.2;
                    rendered[index + 3] = colour.3;

                    if x1 == x2 && y1 == y2 {
                        break;
                    }

                    let e2 = 2 * err;

                    if e2 > -dy {
                        err -= dy;
                        x1 += sx;
                    }

                    if e2 < dx {
                        err += dx;
                        y1 += sy;
                    }
                }
            }

            GraphicObjects::Triangle(G) => { G.rend(rendered) }
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum Surface {
    Flat(Colour)
}

#[derive(Clone, Debug, Copy)]
#[repr(C)]
pub struct InfLine {
    pub slope: f32,
    pub constant: i32,
}

pub fn crossing_point(x: i32, light: Option<InfLine>) -> Option<i32> {
    match light {
        Some(line) => unsafe {
            let y = line.slope * (x as f32) + (line.constant as f32);
            Some(floorf32(y) as i32)
        },
        None => None
    }
}

pub fn line_between_points(p1: DiCoordinate, p2: DiCoordinate) -> Option<InfLine> {
    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;

    //RIP The Slope Checker

    let slope = dy as f32 / dx as f32;

    let constant = p1.1;  //(p1.1 as f32 - (slope * p1.0 as f32)) as i32;

    Some(InfLine {slope, constant})
}

fn alpha_blend(foreground: (u8, u8, u8, u8), background: (u8, u8, u8, u8)) -> (u8, u8, u8, u8) {
    let fg_alpha = foreground.3 as f64 / 255.0; // Foreground alpha normalized to [0, 1]
    let bg_alpha = background.3 as f64 / 255.0; // Background alpha normalized to [0, 1]

    // Composite alpha
    let out_alpha = fg_alpha + bg_alpha * (1.0 - fg_alpha);

    if out_alpha == 0.0 {
        return (0, 0, 0, 0); // Fully transparent
    }

    // Blend each channel
    let blend_channel = |fg: u8, bg: u8| {
        ((fg as f64 * fg_alpha + bg as f64 * bg_alpha * (1.0 - fg_alpha)) / out_alpha).round() as u8
    };

    let r = blend_channel(foreground.0, background.0);
    let g = blend_channel(foreground.1, background.1);
    let b = blend_channel(foreground.2, background.2);

    // Return the blended color
    (
        r,
        g,
        b,
        (out_alpha * 255.0).round() as u8, // Convert output alpha back to [0, 255]
    )
}

fn paint_it(rendered: &mut Visual, surface: Surface, cord: &DiCoordinate) {
    let index = indexify(cord);

    match surface {
        Flat(colour) => {
            let updated = alpha_blend(colour,
                                      (
                                          rendered[index],
                                          rendered[index + 1],
                                          rendered[index + 2],
                                          rendered[index + 3], ));

            rendered[index] = updated.0;
            rendered[index + 1] = updated.1;
            rendered[index + 2] = updated.2;
            rendered[index + 3] = updated.3;
        }
    }
}




