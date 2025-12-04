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

## Progress ## 

Did some brainstorming with chatgpt on how to approach
Mainly just asking how to simulate fluid dynamics because i have no idea

Came up with:

LBM D2Q9 grid representation for the simulation math + data
Pixels for window and visualization

Starting out with generating an "infinite" lattice
it just wraps around like that OG mario game with the turtles
That way i don't have to worry about "clean air" and "dirty air",
or worry about object interactions with the fluid sim (yet)
