
mod vec2d;

use graphics::types::Color;
use piston::UpdateArgs;
use rand::Rng;
use rand::seq::IteratorRandom;
pub use vec2d::Vec2d;

pub struct Simulation {
    area: Area,
    particles: Vec<Particle>,
    gravity: Vec2d,
}

impl Simulation {
    pub fn new(area_size: Vec2d, gravity: Vec2d, particles: Vec<ParticleTemplate>) -> Simulation {
        let mut rng = rand::thread_rng();

        let area = Area{ size: area_size };

        let count: usize = particles.iter()
            .map(|pt| pt.count)
            .sum();
        let max_size = particles.iter()
            .map(|pt| pt.radius)
            .max()
            .unwrap();

        let width = (area_size.x / (2.0 * max_size)) as u32;
        let height = (area_size.y / (2.0 * max_size)) as u32;

        let positions: Vec<Vec2d> = (0..width)
            .map(|i| {
                (0..height).map(|j| (i, j))
            })
            .flatten()
            .choose_multiple(&mut rng, count).iter()
            .map(|(i, j)| {
                Vec2d::new(i as f64 * (area_size.x / width as f64) + max_size, j as f64 * (area_size.y / width as f64) + max_size)
            }).collect();
        let mut positions = positions.iter();

        let particles: Vec<Particle> = particles.iter()
            .map(|pt| {
                (0..pt.count).map(|_| {
                    let rad = rng.gen::<f64>();
                    Particle {
                        pos: *positions.next().unwrap(),
                        radius: pt.radius,
                        color: pt.color,
                        vel: Vec2d::new(pt.vel * rad.sin(), pt.vel * rad.cos()),
                        acc: Vec2d::new(0.0, 0.0),
                    }
                })
            })
            .flatten()
            .collect();

        Simulation {
            area,
            particles,
            gravity,
        }
    }

    pub fn particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    pub fn area(&self) -> &Area {
        &self.area
    }

    pub fn update(&mut self, args: UpdateArgs) {
        todo!()
    }
}

pub struct Particle {
    pub pos: Vec2d,
    pub radius: f64,
    pub color: Color,
    vel: Vec2d,
    acc: Vec2d,
}

pub struct ParticleTemplate {
    pub radius: f64,
    pub vel: f64,
    pub color: Color,
    pub count: u32,
}

pub struct Area {
    pub size: Vec2d,
}