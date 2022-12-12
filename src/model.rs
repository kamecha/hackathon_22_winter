use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};

pub struct Vertex {
    pub x: f64,
    pub y: f64,
}

pub struct Edge {
    pub start: i64,
    pub end: i64,
}

pub trait Object {
    fn render(&mut self, args: &RenderArgs);
    fn update(&mut self, args: &UpdateArgs);
}

pub struct Rod {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub speed: f64,
}

impl Object for Rod {
    fn render(&mut self, args: &RenderArgs) {
        let gl = &mut GlGraphics::new(OpenGL::V3_2);

        gl.draw(args.viewport(), |c, gl| {
            use graphics::*;

            for edge in &self.edges {
                let start = &self.vertices[edge.start as usize];
                let end = &self.vertices[edge.end as usize];

                line(
                    [1.0, 0.0, 0.0, 1.0],
                    1.0,
                    [start.x, start.y, end.x, end.y],
                    c.transform,
                    gl,
                );
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {}
}

impl Rod {
    pub fn right(&mut self) {
        for vertex in &mut self.vertices {
            vertex.x += self.speed;
        }
    }
    pub fn left(&mut self) {
        for vertex in &mut self.vertices {
            vertex.x -= self.speed;
        }
    }
}

pub struct Square {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
}

impl Object for Square {
    fn render(&mut self, args: &RenderArgs) {
        let gl = &mut GlGraphics::new(OpenGL::V3_2);

        gl.draw(args.viewport(), |c, gl| {
            use graphics::*;

            for edge in &self.edges {
                let start = &self.vertices[edge.start as usize];
                let end = &self.vertices[edge.end as usize];

                line(
                    [1.0, 0.0, 0.0, 1.0],
                    1.0,
                    [start.x, start.y, end.x, end.y],
                    c.transform,
                    gl,
                );
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {}
}
