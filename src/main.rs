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

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            let gl = &mut GlGraphics::new(OpenGL::V3_2);
            use graphics::*;
            clear([0.0; 4], gl);
            // render squares and rod
            for object in &mut object_manager.objects {
                match object {
                    ObjectType::Square(square) => {
                        square.render(&args);
                    }
                    ObjectType::Rod(rod) => {
                        rod.render(&args);
                    }
                }
            }
        }
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(keyboard::Key::Right) => {
                    for object in &mut object_manager.objects {
                        match object {
                            ObjectType::Rod(rod) => {
                                rod.right();
                            }
                            _ => {}
                        }
                    }
                }
                Button::Keyboard(keyboard::Key::Left) => {
                    for object in &mut object_manager.objects {
                        match object {
                            ObjectType::Rod(rod) => {
                                rod.left();
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        if let Some(args) = e.update_args() {
            // root.update(&args);
        }
    }
}
