use crate::engine::{Area, Particle};

mod sweep_collider;
mod box_collider;

pub use sweep_collider::SweepCollider;
pub use box_collider::BoxCollider;

/// A trait for detecting collisions
pub trait CollisionDetector {
    /// This function should not only detect collisions, but also process them
    fn process_collisions(&self, area: &Area, particles: &mut Vec<Particle>);
}

/// An enum representing all cases of collisions between particles and the box
pub enum BoxCollision<'a> {
    Top(&'a mut Particle),
    Right(&'a mut Particle),
    Bottom(&'a mut Particle),
    Left(&'a mut Particle)
}

/// An enum representing collisions
pub enum Collision<'a> {
    Box(BoxCollision<'a>),
    Particle(&'a mut Particle, &'a mut Particle)
}

/// A trait which implements collision processing for defining custom on-collision behaviour
pub trait CollisionProcessor {
    /// A method for processing collisions.
    ///
    /// The default implementation uses perfectly elastic collision physics
    fn process_collision(&self, collision: Collision) {
        match collision {
            Collision::Box(collision) => {
                match collision {
                    BoxCollision::Top(p) => if p.vel.y > 0.0 { p.vel.y *= -1.0; }
                    BoxCollision::Right(p) => if p.vel.x > 0.0 { p.vel.x *= -1.0; }
                    BoxCollision::Bottom(p) => if p.vel.y < 0.0 { p.vel.y *= -1.0; }
                    BoxCollision::Left(p) => if p.vel.x < 0.0 { p.vel.x *= -1.0; }
                }
            }
            Collision::Particle(p1, p2) => {
                let m1 = p1.radius * p1.radius;
                let m2 = p2.radius * p2.radius;
                let v1 = (p1.vel * (m1 - m2) + 2.0 * m2 * p2.vel) / (m1 + m2);
                let v2 = (p2.vel * (m2 - m1) + 2.0 * m1 * p1.vel) / (m1 + m2);
                p1.vel = v1;
                p2.vel = v2;
            }
        }
    }
}

/// Basic collision processor which uses the default implementation of [CollisionProcessor]
pub struct BasicCollisionProcessor;

impl CollisionProcessor for BasicCollisionProcessor {}
