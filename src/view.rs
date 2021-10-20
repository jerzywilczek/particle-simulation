use std::collections::HashMap;
use graphics::{Context, Graphics, Ellipse, Rectangle};
use graphics::types::Color;
use itertools::Itertools;
use crate::engine::{Simulation, Vec2d, Collider};

pub const BLACK: Color = [0.0, 0.0, 0.1, 1.0];

pub struct RendererSettings {
    pub offset: Vec2d,
    pub background_color: Color,
    pub border_color: Color,
    pub border_size: f64,
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

    pub fn draw<G: Graphics, C: Collider>(&self, simulation: &Simulation<C>, c: Context, g: &mut G) {
        use graphics::clear;

        clear(self.settings.background_color, g);

        for particle in simulation.particles() {
            let [x, y] = (
                Vec2d::new(particle.pos.x, simulation.area().size.y - particle.pos.y) - Vec2d::new(particle.radius, particle.radius) + self.settings.offset
            ).to_arr();
            let rect = [
                x, y,
                2.0 * particle.radius, 2.0 * particle.radius
            ];

            Ellipse::new(particle.color).draw(rect, &c.draw_state, c.transform, g);
        }

        Rectangle::new_border(self.settings.border_color, self.settings.border_size)
            .draw(
                [
                    self.settings.offset.x - self.settings.border_size, self.settings.offset.y - self.settings.border_size,
                    simulation.area().size.x + self.settings.border_size * 2.0, simulation.area().size.y + self.settings.border_size * 2.0
                ],
                &c.draw_state, c.transform, g
            );
    }
}