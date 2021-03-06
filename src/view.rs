use graphics::{Context, Graphics, Ellipse, Rectangle};
use graphics::types::Color;
use crate::engine::{Simulation, Vec2d, CollisionDetector};


pub mod colors {
    use graphics::types::Color;

    /// This is very sketchy but it works
    macro_rules! hex {
        ($color:expr) => {[
            (($color & 0xff0000) >> 16) as f32 / 256.0,
            (($color & 0x00ff00) >> 8) as f32 / 256.0,
            (($color & 0x0000ff) >> 0) as f32 / 256.0,
            1.0
        ]};
    }

    pub const STEEL_BLUE: Color = hex!(0x2D7F9D);
    pub const LIGHT_STEEL_BLUE: Color = hex!(0xA4C9D7);
    pub const BURLY_WOOD: Color = hex!(0xE4CA99);
    pub const PALE_VIOLET_RED: Color = hex!(0xDC7684);
}

/// A struct for holding settings for the renderer
pub struct RendererSettings {
    /// Offset from the top-left corner
    pub offset: Vec2d,
    pub background_color: Color,
    pub border_color: Color,
    pub border_size: f64,
}

impl RendererSettings {
    /// Creates the struct with default settings
    pub fn new() -> RendererSettings {
        RendererSettings {
            offset: Vec2d::new(10.0, 10.0),
            background_color: colors::LIGHT_STEEL_BLUE,
            border_color: colors::STEEL_BLUE,
            border_size: 2.0
        }
    }
}

/// Does rendering
pub struct Renderer {
    settings: RendererSettings,
}

impl Renderer {
    /// Creates a new Renderer
    pub fn new(settings: RendererSettings) -> Renderer {
        Renderer {
            settings,
        }
    }

    /// Draws the simulation on screen
    pub fn draw<G: Graphics, C: CollisionDetector>(&self, simulation: &Simulation<C>, c: Context, g: &mut G) {
        use graphics::clear;

        // clear the screen
        clear(self.settings.background_color, g);

        // draw particles
        for particle in simulation.particles() {
            let [x, y] = <[f64; 2]>::from(
                Vec2d::new(particle.pos.x, simulation.area().size.y - particle.pos.y) - Vec2d::new(particle.radius, particle.radius) + self.settings.offset
            );
            let rect = [
                x, y,
                2.0 * particle.radius, 2.0 * particle.radius
            ];

            Ellipse::new(particle.color).draw(rect, &c.draw_state, c.transform, g);
        }

        // draw the box
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