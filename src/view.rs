use std::collections::HashMap;
use graphics::{Context, Graphics, Ellipse, Rectangle};
use graphics::types::Color;
use itertools::Itertools;
use crate::engine::{Simulation, Vec2d};

pub const BLACK: Color = [0.0, 0.0, 0.1, 1.0];

pub struct RendererSettings {
    pub offset: Vec2d,
    pub background_color: Color,
    pub border_color: Color,
    pub border_size: f64,
    pub particle_border_size: Option<f64>,
}

pub struct Renderer {
    settings: RendererSettings,
}

impl Renderer {
    pub fn new(settings: RendererSettings) -> Renderer {
        Renderer {
            settings,
        }
    }

    pub fn draw<G: Graphics>(&self, simulation: &Simulation, c: &Context, g: &mut G) {
        use graphics::clear;

        clear(self.settings.background_color, g);

        let circles: HashMap<Color, Ellipse> = simulation.particles().iter()
            .map(|p| p.color)
            .unique()
            .map(|c| (c, Ellipse::new(c)))
            .collect();

        for particle in simulation.particles() {
            let [x, y] = (particle.pos - Vec2d::new(particle.radius, particle.radius) + self.settings.offset).to_arr();
            let rect = [
                x, y,
                2.0 * particle.radius, 2.0 * particle.radius
            ];

            circles.get(&particle.color).unwrap().draw(rect, &c.draw_state, c.transform, g);
        }

        Rectangle::new_border(self.settings.border_color, self.settings.border_size)
            .draw(
                [
                    self.settings.offset.x, self.settings.offset.y,
                    simulation.area().size.x, simulation.area().size.y
                ],
                &c.draw_state, c.transform, g
            );
    }
}