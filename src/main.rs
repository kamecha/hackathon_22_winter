mod model;
mod object_manager;

use glutin_window::GlutinWindow;
use model::*;
use object_manager::*;
use opengl_graphics::GlGraphics;
use opengl_graphics::OpenGL;
use piston::input::UpdateEvent;
use piston::keyboard;
use piston::{Button, EventSettings, Events, PressEvent, RenderEvent, WindowSettings};

fn main() {
    let opengl = OpenGL::V3_2;
    // Create an Glutin window.
    let cell_base = 30.0;
    let window_base: f64 = cell_base * 3.0;
    let mut window: GlutinWindow =
        WindowSettings::new("apinning-square", [16.0 * window_base, 9.0 * window_base])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut object_manager = ObjectManager::new();
    object_manager.generate_object_from_csv("src/tmp.csv", window_base);

    // Create a rod
    // let mut root = Rod {
    //     vertices: vec![Vertex { x: 100.0, y: 100.0 }, Vertex { x: 200.0, y: 200.0 }],
    //     edges: vec![Edge { start: 0, end: 1 }],
    //     speed: 20.0,
    // };
    //
    // object_manager.add(Box::new(root));
    //
    // // Create a squares for map
    // // let mut squares = vec![];
    // for i in 0..5 {
    //     for j in 0..5 {
    //         object_manager.add(Box::new(Square {
    //             vertices: vec![
    //                 Vertex {
    //                     x: i as f64 * 100.0,
    //                     y: j as f64 * 100.0,
    //                 },
    //                 Vertex {
    //                     x: i as f64 * 100.0 + 100.0,
    //                     y: j as f64 * 100.0,
    //                 },
    //                 Vertex {
    //                     x: i as f64 * 100.0 + 100.0,
    //                     y: j as f64 * 100.0 + 100.0,
    //                 },
    //                 Vertex {
    //                     x: i as f64 * 100.0,
    //                     y: j as f64 * 100.0 + 100.0,
    //                 },
    //             ],
    //             edges: vec![
    //                 Edge { start: 0, end: 1 },
    //                 Edge { start: 1, end: 2 },
    //                 Edge { start: 2, end: 3 },
    //                 Edge { start: 3, end: 0 },
    //             ],
    //         }));
    //     }
    // }
    //
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            let gl = &mut GlGraphics::new(OpenGL::V3_2);
            use graphics::*;
            clear([0.0; 4], gl);
            // render squares
            for object in &mut object_manager.objects {
                object.render(&args);
            }
        }
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(keyboard::Key::Right) => {
                    // object_manager.objects.iter_mut().for_each(|object| {
                    // match object {
                    // move rod
                    // Rod { vertices, .. } => {}
                    // _ => {}
                    // }
                    // });
                    // root.right();
                }
                Button::Keyboard(keyboard::Key::Left) => {
                    // root.left();
                }
                _ => {}
            }
        }
        if let Some(args) = e.update_args() {
            // root.update(&args);
        }
    }
}
