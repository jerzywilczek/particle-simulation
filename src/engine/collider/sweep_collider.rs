use std::collections::HashSet;
use crate::engine::{Area, BoxCollider, CollisionDetector, Particle};
use crate::engine::collider::{Collision, CollisionProcessor};

/// A collider which detects collisions between pairs of particles and between particles and the box
pub struct SweepCollider<C: CollisionProcessor, B: CollisionProcessor> {
    box_collider: BoxCollider<B>,
    processor: C,
}

impl<C: CollisionProcessor, B: CollisionProcessor> SweepCollider<C, B> {
    /// Creates a new SweepCollider which uses the given processor and BoxCollector
    pub fn new(processor: C, box_collider: BoxCollider<B>) -> SweepCollider<C, B> {
        SweepCollider {
            box_collider,
            processor,
        }
    }
}

impl<C: CollisionProcessor, B: CollisionProcessor> CollisionDetector for SweepCollider<C, B> {
    fn process_collisions(&self, area: &Area, particles: &mut Vec<Particle>) {
        self.box_collider.process_collisions(area, particles);

        /// Event is either the beginning or the end of a particle
        enum Event {
            Begin(f64, usize),
            End(f64, usize)
        }

        // create vec holding events
        let mut events: Vec<Event> = Vec::with_capacity(2 * particles.len());
        for (i, particle) in particles.iter().enumerate() {
            events.push(Event::Begin(particle.pos.x - particle.radius, i));
            events.push(Event::End(particle.pos.x + particle.radius, i));
        }

        // sort the events
        events.sort_unstable_by(|e1, e2| {
            let a = match e1 {
                Event::Begin(a, _) => a,
                Event::End(a, _) => a,
            };
            let b = match e2 {
                Event::Begin(b, _) => b,
                Event::End(b, _) => b,
            };
            a.partial_cmp(b).unwrap()
        });

        // perform the sweeping by processing events in order
        let mut open = HashSet::new();
        for e in events {
            match e {
                Event::Begin(_, i) => {
                    for &j in &open {
                        let (p1, p2): (&Particle, &Particle) = (
                            particles.get(i).unwrap(),
                            particles.get(j).unwrap()
                        );

                        // check whether p1 and p2 collide
                        if (p1.pos - p2.pos).len() < p1.radius + p2.radius {
                            // extract mutable references
                            let (i, j) = (i.min(j), i.max(j));
                            let mut iter = particles.iter_mut();
                            let p1 = iter.nth(i).unwrap();
                            let p2 = iter.nth(j - i - 1).unwrap();

                            // process the collision
                            self.processor.process_collision(Collision::Particle(p1, p2))
                        }
                    }
                    open.insert(i);
                }
                Event::End(_, i) => {
                    open.remove(&i);
                }
            }
        }

    }
}