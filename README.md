# particle-simulation
Ball physics simulator based on fully elastic collisions and a sweep line algorithm for collision detection.

## Compiling and running

First, you have to [install rustc and cargo](https://www.rust-lang.org/tools/install). Then clone this repository, navigate to the cloned folder and run
```
cargo run --release
```

## Configuration

Until #4 is resolved, all configuration has to be done in source code. To do that, add wanted [`Particle templates`](https://github.com/jerzywilczek/particle-simulation/blob/8f87f040912c0d25f0410d0b9b1e989b8b36e536/src/engine/mod.rs#L110) to the `Vec` in [`Simulation`'s constructor](https://github.com/jerzywilczek/particle-simulation/blob/8f87f040912c0d25f0410d0b9b1e989b8b36e536/src/lib.rs#L16) and then [recompile and run](#compiling-and-running) the cargo project.
