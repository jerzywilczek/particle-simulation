//! This module contains everything needed to run the simulation
mod vec2d;
mod collider;

use std::f64::consts::PI;
use graphics::types::Color;
use piston::UpdateArgs;
use rand::Rng;
use rand::seq::IteratorRandom;
pub use vec2d::Vec2d;
pub use collider::{BoxCollider, CollisionDetector, SweepCollider, BasicCollisionProcessor};

/// A struct for holding the simulation
pub struct Simulation<C: CollisionDetector> {
    area: Area,
    particles: Vec<Particle>,
    gravity: Vec2d,
    collider: C,
}

impl<C: CollisionDetector> Simulation<C> {
    /// Creates a new simulation with given parameters
    ///
    /// If templates specify more particles than the area can fit, the rest of the particles will be omitted
    pub fn new(area_size: Vec2d, gravity: Vec2d, templates: Vec<ParticleTemplate>, collider: C) -> Simulation<C> {
        let mut rng = rand::thread_rng();

        let area = Area{ size: area_size };

        let count: usize = templates.iter()
            .map(|pt| pt.count)
            .sum();
        let max_size = templates.iter()
            .map(|pt| pt.radius)
            .max_by(|a,b | a.partial_cmp(b).unwrap())
            .unwrap_or(1.0);

        let width = (area_size.x / (2.0 * max_size)) as usize;
        let height = (area_size.y / (2.0 * max_size)) as usize;

        let mut positions = Vec::with_capacity(width * height);
        for i in 0..width {
            for j in 0..height {
                positions.push(Vec2d::new(
                    i as f64 * (area_size.x / width as f64) + max_size,
                    j as f64 * (area_size.y / height as f64) + max_size
                ));
            }
        }
        let positions = positions.iter().choose_multiple(&mut rng, count);
        let mut positions = positions.iter();

        let mut particles = Vec::with_capacity(count);
        'outer: for pt in templates {
            for _ in 0..pt.count {
                let rad = rng.gen::<f64>() * 2.0 * PI - PI;
                if let Some(&&pos) = positions.next() {
                    particles.push(Particle {
                        pos,
                        radius: pt.radius,
                        color: pt.color,
                        vel: Vec2d::new(pt.vel * rad.sin(), pt.vel * rad.cos()),
                        acc: Vec2d::new(0.0, 0.0),
                    });
                } else {
                    break 'outer;
                }
            }
        }

        Simulation {
            area,
            particles,
            gravity,
            collider,
        }
    }

    /// Returns a reference to the particles
    pub fn particles(&self) -> &Vec<Particle> {
        &self.particles
    }

    /// Returns a reference to the area
    pub fn area(&self) -> &Area {
        &self.area
    }

    /// Updates state of the simulation
    pub fn update(&mut self, args: UpdateArgs) {
        for particle in &mut self.particles {
            particle.pos += particle.vel * args.dt;
            particle.vel += self.gravity * args.dt;
        }
        self.collider.process_collisions(&self.area, &mut self.particles);
    }
}

#[derive(Clone)]
/// Struct describing a single particle
pub struct Particle {
    pub pos: Vec2d,
    pub radius: f64,
    pub color: Color,
    vel: Vec2d,
    acc: Vec2d,
}

/// Struct for describing a kind of particles which will be spawned in the simulation
pub struct ParticleTemplate {
    pub radius: f64,
    pub vel: f64,
    pub color: Color,
    pub count: usize,
}

/// Struct for holding area dimensions
pub struct Area {
    pub size: Vec2d,
}