//! Simulation module
//! 
//! All core functionality, simulation logic, etc. is managed in here
//! 
//! # Fields
//! width: how wide the grid is 
//! height: how tall the grid is
//! field: placeholder for grid
//! 
//! # How does this work?
//! 
//! Per cell, we store density, velocity, and a tiny probability distribution
//! (aka where stuff is moving around it)
//! That means we have 9 values, f0 -> f8 representing cardinal directions,
//! and the volume of fluid moving in those directions
//! Using them, we determine density and velocity
//! 
//! Next, we're managing collisions and streaming.
//! For collisions, we find an equilibrium distribution and "relax" towards it
//! For streaming, after calculations we move the effects to the surrounding cells

mod lattice;
mod collision;
mod streaming;
mod metrics;
mod boundary;

pub struct Simulation {
    width: usize,
    height: usize,
    omega: f64,
    f: Vec<[f64; lattice::Q]>,
}

/// Creates a new `Simulation` instance 
///
/// # Fields:
/// width: how wide the grid is
/// height: how tall the grid is
///
/// # Returns
/// A new simulation with a density of '1' for each cell of the grid
/// 
impl Simulation {


}