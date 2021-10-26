use crate::engine::collider::{BoxCollision, Collision, CollisionProcessor};
use crate::engine::{Area, CollisionDetector, Particle};

/// A collider which only detects collisions between particles and the box
pub struct BoxCollider<C: CollisionProcessor> {
    processor: C,
}

impl<C: CollisionProcessor> BoxCollider<C> {
    /// Creates a new BoxCollider which uses the given processor
    pub fn new(processor: C) -> BoxCollider<C> {
        BoxCollider {
            processor,
        }
    }
}

impl<C: CollisionProcessor> CollisionDetector for BoxCollider<C> {
    fn process_collisions(&self, area: &Area, particles: &mut Vec<Particle>) {
        for particle in particles {
            if particle.pos.x - particle.radius <= 0.0 {
                self.processor.process_collision(Collision::Box(BoxCollision::Left(particle)));
            }
            else if particle.pos.x + particle.radius >= area.size.x {
                self.processor.process_collision(Collision::Box(BoxCollision::Right(particle)));
            }
            if particle.pos.y - particle.radius <= 0.0 {
                self.processor.process_collision(Collision::Box(BoxCollision::Bottom(particle)));
            }
            else if particle.pos.y + particle.radius >= area.size.y {
                self.processor.process_collision(Collision::Box(BoxCollision::Top(particle)));
            }
        }
    }
}