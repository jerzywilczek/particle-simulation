use crate::engine::{Area, Particle};

mod sweep_collider;

pub use sweep_collider::SweepCollider;

pub trait Collider {
    fn process_collisions(&self, area: &Area, particles: &mut Vec<Particle>);
}

pub struct BoxCollider;

impl Collider for BoxCollider {
    fn process_collisions(&self, area: &Area, particles: &mut Vec<Particle>) {
        for particle in particles {
            if particle.pos.x - particle.radius <= 0.0 && particle.vel.x < 0.0 {
                particle.vel.x *= -1.0;
            }
            else if particle.pos.x + particle.radius >= area.size.x && particle.vel.x > 0.0 {
                particle.vel.x *= -1.0;
            }
            if particle.pos.y - particle.radius <= 0.0 && particle.vel.y < 0.0 {
                particle.vel.y *= -1.0;
            }
            else if particle.pos.y + particle.radius >= area.size.y && particle.vel.y > 0.0 {
                particle.vel.y *= -1.0;
            }
        }
    }
}
