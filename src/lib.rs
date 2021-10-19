use glutin_window::GlutinWindow;
use graphics::Graphics;
use opengl_graphics::GlGraphics;
use piston::{Events, RenderEvent};

mod engine;
mod view;

pub fn run<G: Graphics>(mut window: GlutinWindow, mut events: Events, mut gl: G) {
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {

            });
        }
    }
}
