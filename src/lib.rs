use glutin_window::GlutinWindow;
use opengl_graphics::GlGraphics;
use piston::{Events, RenderEvent, UpdateEvent, Window};
use crate::engine::{SweepCollider, ParticleTemplate, Simulation, Vec2d, BoxCollider};
use crate::view::{Renderer, RendererSettings};
use crate::view::colors::*;

mod engine;
mod view;

pub fn run<>(mut window: GlutinWindow, mut events: Events, mut gl: GlGraphics) {
    let [w, h] = <[f64; 2]>::from(window.size());
    let mut simulation = Simulation::new(
        Vec2d::new(w - 20.0, h - 20.0),
        Vec2d::new(0.0, 0.0),
        vec![
            ParticleTemplate {
                radius: 10.0,
                vel: 50.0,
                color: PALE_VIOLET_RED,
                count: 50
            },
            ParticleTemplate {
                radius: 5.0,
                vel: 200.0,
                color: BURLY_WOOD,
                count: 50
            }
        ],
        SweepCollider::new(BoxCollider),
    );

    let renderer = Renderer::new(RendererSettings::new());

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
