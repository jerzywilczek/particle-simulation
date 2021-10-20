
mod vec2d;
mod collider;

use std::f64::consts::PI;
use graphics::types::Color;
use piston::UpdateArgs;
use rand::Rng;
use rand::seq::IteratorRandom;
pub use vec2d::Vec2d;
pub use collider::{BoxCollider, Collider, SweepCollider};

pub struct Simulation<C: Collider> {
    area: Area,
    particles: Vec<Particle>,
    gravity: Vec2d,
    collider: C,
}

impl<C: Collider> Simulation<C> {
    pub fn new(area_size: Vec2d, gravity: Vec2d, templates: Vec<ParticleTemplate>, collider: C) -> Simulation<C> {
        let mut rng = rand::thread_rng();

        let area = Area{ size: area_size };

        let count: usize = templates.iter()
            .map(|pt| pt.count)
            .sum();
        let max_size = templates.iter()
            .map(|pt| pt.radius)
            .max_by(|a,b | a.partial_cmp(b).unwrap())
            .unwrap();

        let width = (area_size.x / (2.0 * max_size)) as u32;
        let height = (area_size.y / (2.0 * max_size)) as u32;

        let positions: Vec<Vec2d> = (0..width)
            .map(|i| {
                (0..height).map(move |j| (i, j))
            })
            .flatten()
            .choose_multiple(&mut rng, count).iter()
            .map(|(i, j)| {
                Vec2d::new(*i as f64 * (area_size.x / width as f64) + max_size, *j as f64 * (area_size.y / height as f64) + max_size)
            }).collect();
        let mut positions = positions.iter();

        let mut particles = Vec::with_capacity(count);
        for pt in templates {
            for _ in 0..pt.count {
                let rad = rng.gen::<f64>() * 2.0 * PI - PI;
                particles.push(Particle {
                    pos: *positions.next().expect("Box is not big enough for all particles"),
                    radius: pt.radius,
                    color: pt.color,
                    vel: Vec2d::new(pt.vel * rad.sin(), pt.vel * rad.cos()),
                    acc: Vec2d::new(0.0, 0.0),
                });
            }
        }

        Simulation {
            area,
            particles,
            gravity,
            collider,
        }
    }

    pub fn particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    pub fn area(&self) -> &Area {
        &self.area
    }

    pub fn update(&mut self, args: UpdateArgs) {
        for particle in &mut self.particles {
            particle.pos += particle.vel * args.dt;
            particle.vel += self.gravity * args.dt;
        }
        self.collider.process_collisions(&self.area, &mut self.particles);
    }
}

#[derive(Clone)]
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
    pub count: usize,
}

pub struct Area {
    pub size: Vec2d,
}