use glutin_window::GlutinWindow;
use opengl_graphics::GlGraphics;
use piston::{Events, RenderEvent};

mod engine;

pub fn run(mut window: GlutinWindow, mut events: Events, mut gl: GlGraphics) {
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                const BLACK: [f32; 4] = [0.0, 0.0, 0.1, 1.0];

                clear(BLACK, g);
            });
        }
    }
}
