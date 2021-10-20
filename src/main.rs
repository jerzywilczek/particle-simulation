extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{EventLoop, Events, EventSettings, WindowSettings};

fn main() {
    let opengl = OpenGL::V3_2;

    const WINDOW_SIZE: [u32; 2] = [512, 512];

    let window: GlutinWindow = WindowSettings::new("Particle simulation", WINDOW_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .samples(8)
        .vsync(true)
        .build()
        .expect("Couldn't create window");

    let gl = GlGraphics::new(opengl);
    let events = Events::new(EventSettings::new()
        .ups(60)
        .max_fps(60)
    );

    particle_simulation::run(window, events, gl);
}
