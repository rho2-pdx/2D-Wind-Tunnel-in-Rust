# 2D Wind Tunnel in Rust
**Author:** Ryan Houlberg - rho2@pdx.edu

This program uses the Lattice Boltzmann Method (LBM) with a D2Q9 lattice and BGK collision model to simulate fluid dynamics in 2D.
It is an instrumental simulation as it evolves over time and displays state via its visualizer

## Goals ##

- Implement D2Q9 lattice with BGK collision math on 2D grid
- Add in solid objects defined as bitmap masks
- Visualize the flow with the "pixels" crate
- Allow parameter control at runtime:
  - Simulation speed
  - Inlet velocity
  - Fluid Type (presets)
- Compute and offer drag and lift values


## How to build ##
- cargo run
- cargo test (to run tests)

## Lessons ##
Don't do a coding project on a topic that you don't understand at all
Especially when it's an advanced physics problem

## Resources Used ##
window management + pixel displaying
https://github.com/parasyte/pixels/blob/21bae15f854186598e21ad508f4068a0ebb26d1c/examples/conway/src/main.rs
https://github.com/parasyte/pixels/tree/21bae15f854186598e21ad508f4068a0ebb26d1c/examples/invaders
https://www.youtube.com/watch?v=alhpH6ECFvQ
https://github.com/CodingTrain/Suggestion-Box/issues/178
https://physics.weber.edu/schroeder/fluids/
https://www.reddit.com/r/CFD/comments/1gfnkg3/rust_lbm_solver_on_the_gpu/
https://www.oatext.com/lattice-boltzmann-modeling-for-mass-and-velocity-fields-of-casting-flows.php
https://journals.aps.org/pr/abstract/10.1103/PhysRev.94.511
https://github.com/emoon/rust_minifb