use criterion::{black_box, criterion_group, criterion_main, Criterion};
use piston::UpdateArgs;
use particle_simulation::engine::{BasicCollisionProcessor, BoxCollider, ParticleTemplate, Simulation, SweepCollider, Vec2d};


pub fn sweep_collider(c: &mut Criterion) {
    let area_size = Vec2d::new(200.0, 200.0);
    let gravity = Vec2d::new(0.0, 0.0);
    let black = [0.0, 0.0, 0.0, 1.0];
    let templates = vec![
        ParticleTemplate {
            radius: 10.0,
            vel: 50.0,
            color: black,
            count: 50
        },
        ParticleTemplate {
            radius: 15.0,
            vel: 200.0,
            color: black,
            count: 50
        }
    ];

    let mut simulation = Simulation::new(
        area_size,
        gravity,
        templates,
        SweepCollider::new(BasicCollisionProcessor, BoxCollider::new(BasicCollisionProcessor))
    );

    c.bench_function("sweep collider", |b| b.iter(|| simulation.update(black_box(UpdateArgs{dt: 0.5}))));
}

criterion_group!(benches, sweep_collider);
criterion_main!(benches);