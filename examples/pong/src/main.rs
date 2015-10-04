extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::input::Event;
use piston::event_loop::{EventLoop,Events};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

const OPEN_GL: OpenGL = OpenGL::V2_1;

fn main() {
    let window: GlutinWindow = WindowSettings::new("Pong", [800,600])
        .exit_on_esc(true)
        .opengl(OPEN_GL)
        .build().unwrap();

    let mut gl = GlGraphics::new(OPEN_GL);

    let pad = graphics::rectangle::Rectangle::new([1.0,0.0,0.0,1.0]);
        
    for e in window.events().ups(60).max_fps(60) {
        match e {
            Event::Render(args) => {
                graphics::clear([0.0,0.0,0.0,1.0],&mut gl);

                gl.draw(args.viewport(), |c, g| {
                    pad.draw([50.0,50.0,25.0,97.0],
                             &c.draw_state,
                             c.transform,
                             g);
                });
            }
            Event::Update(args) => {
            }
            _ => {}
        }
    }
}
