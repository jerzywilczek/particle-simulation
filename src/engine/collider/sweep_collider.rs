use std::collections::HashSet;
use crate::engine::{Area, BoxCollider, Collider, Particle};

pub struct SweepCollider {
    box_collider: BoxCollider
}

impl SweepCollider {
    pub fn new(box_collider: BoxCollider) -> SweepCollider {
        SweepCollider {
            box_collider,
        }
    }

    fn collide(&self, p1: &mut Particle, p2: &mut Particle) {
        let m1 = p1.radius * p1.radius;
        let m2 = p2.radius * p2.radius;
        let v1 = (p1.vel * (m1 - m2) + p2.vel * 2.0 * m2) / (m1 + m2);
        let v2 = (p2.vel * (m2 - m1) + p1.vel * 2.0 * m1) / (m1 + m2);
        p1.vel = v1;
        p2.vel = v2;
    }
}

impl Collider for SweepCollider {
    fn process_collisions(&self, area: &Area, particles: &mut Vec<Particle>) {
        self.box_collider.process_collisions(area, particles);

        enum Event {
            Begin,
            End
        }

        let mut events: Vec<(f64, usize, Event)> = Vec::with_capacity(2 * particles.len());
        for (i, particle) in particles.iter().enumerate() {
            events.push((particle.pos.x - particle.radius, i, Event::Begin));
            events.push((particle.pos.x + particle.radius, i, Event::End));
        }
        events.sort_unstable_by(|(a, _, _), (b, _, _)| a.partial_cmp(b).unwrap());

        let mut filtered: HashSet<usize> = HashSet::new();
        let mut open = HashSet::new();
        for (_, i, e) in events {
            match e {
                Event::Begin => {
                    open.insert(i);
                    filtered.extend(open.iter());
                }
                Event::End => {
                    open.remove(&i);
                }
            }
        }

        let mut events: Vec<(f64, usize, Event)> = Vec::with_capacity(filtered.len());
        for i in filtered {
            events.push((particles.get(i).unwrap().pos.y - particles.get(i).unwrap().radius, i, Event::Begin));
            events.push((particles.get(i).unwrap().pos.y + particles.get(i).unwrap().radius, i, Event::End));
        }
        events.sort_unstable_by(|(a, _, _), (b, _, _)| a.partial_cmp(b).unwrap());

        let mut open = HashSet::new();
        for (_, i, e) in events {
            match e {
                Event::Begin => {
                    for &j in &open {
                        let mut iter = particles.iter_mut();
                        let p1 = iter.nth(i.min(j)).unwrap();
                        let p2 = iter.nth(i.max(j) - i.min(j) - 1).unwrap();
                        if (p1.pos - p2.pos).len() < p1.radius + p2.radius {
                            self.collide(p1, p2);
                        }
                    }
                    open.insert(i);
                }
                Event::End => {
                    open.remove(&i);
                }
            }
        }
    }
}