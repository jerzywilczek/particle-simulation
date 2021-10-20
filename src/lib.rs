use glutin_window::GlutinWindow;
use graphics::Graphics;
use opengl_graphics::GlGraphics;
use piston::{Events, RenderEvent, UpdateEvent};
use crate::engine::{BoxCollider, ParticleTemplate, Simulation, Vec2d};
use crate::view::{Renderer, RendererSettings};
use crate::view::colors::*;

mod engine;
mod view;

pub fn run<>(mut window: GlutinWindow, mut events: Events, mut gl: GlGraphics) {
    let mut simulation = Simulation::new(
        Vec2d::new(400.0, 400.0),
        Vec2d::new(0.0, -100.0),
        vec![
            ParticleTemplate {
                radius: 10.0,
                vel: 50.0,
                color: [1.0, 0.0, 0.0, 1.0],
                count: 10
            }
        ],
        BoxCollider
    );

    let renderer = Renderer::new(RendererSettings {
        offset: Vec2d::new(10.0, 10.0),
        background_color: BLACK,
        border_color: STEEL_BLUE,
        border_size: 2.0,
    });

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                renderer.draw(&simulation, c, g);
            });
        }

        if let Some(args) = e.update_args() {
            simulation.update(args);
        }
    }
}
