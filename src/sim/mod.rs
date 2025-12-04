//! Simulation module
//! 
//! All core functionality, simulation logic, etc. is managed in here
//! 
//! # Fields
//! width: how wide the grid is 
//! height: how tall the grid is
//! field: placeholder for grid

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

/// Simulation implementation which manages the context of the simulation
impl Simulation {
    /// Creates a new `Simulation` instance 
    ///
    /// # Fields:
    /// width: how wide the grid is
    /// height: how tall the grid is
    ///
    /// # Returns
    /// A new simulation with a density of '1' for each cell of the grid
 
    pub fn new(width: usize, height: usize) -> Self {
        let n = width * height;
        Self {
            width,
            height,
            field: vec![1.0; n],
        }
    }

    /// Moves forward time one "step" in simulation
    ///
    pub fn step(&mut self) {
        for v in &mut self.field {
            *v += 0.001;
        }
    }

    /// Computes the total mass (sum of all cell values) in the simulation grid
    ///
    /// # Returns
    /// A `f32` representing the sum of all densities in the grid
    ///
    pub fn total_mass(&self) -> f32 {
        self.field.iter().sum()
    }
}