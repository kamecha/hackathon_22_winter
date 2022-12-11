use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs, UpdateEvent};
use piston::{event_id, EventSettings, Events, RenderEvent, WindowSettings};

pub struct Vertex {
    pub x: f64,
    pub y: f64,
}

pub trait Object {
    fn render(&mut self, args: &RenderArgs);
    fn update(&mut self, args: &UpdateArgs);
}

pub struct Rod {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<(Vertex, Vertex)>,
}

impl Object for Rod {
    fn render(&mut self, args: &RenderArgs) {
        let gl = &mut GlGraphics::new(OpenGL::V3_2);
        gl.draw(args.viewport(), |c, gl| {
            use graphics::*;

            clear([0.0, 0.0, 0.0, 1.0], gl);
            for edge in &self.edges {
                line(
                    [1.0, 0.0, 0.0, 1.0],
                    1.0,
                    [edge.0.x, edge.0.y, edge.1.x, edge.1.y],
                    c.transform,
                    gl,
                );
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        println!("Updating");
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    // Create an Glutin window.
    let mut window: GlutinWindow = WindowSettings::new("apinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a rod
    let mut rod = Rod {
        vertices: vec![Vertex { x: 0.0, y: 0.0 }, Vertex { x: 100.0, y: 100.0 }],
        edges: vec![(Vertex { x: 0.0, y: 0.0 }, Vertex { x: 100.0, y: 100.0 })],
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            rod.render(&args);
        }
        if let Some(args) = e.update_args() {
            rod.update(&args);
        }
    }
}
