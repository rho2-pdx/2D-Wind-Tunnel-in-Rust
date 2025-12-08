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
    grid: Vec<[f64; lattice::Q]>, 
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

    pub fn new(width: usize, height: usize) -> Self {
        let initial_density = 1.0;
        let initial_velocity_x = 0.0;
        let initial_velocity_y = 0.0;
        
        let cell_equilibrium = lattice::equilibrium_calculator(
            initial_density,
            initial_velocity_x,
            initial_velocity_y,
        );
        
        let size = width * height;
        let mut grid = Vec::with_capacity(size); // a 1D representation of 2D grid
        for _ in 0..size {
            grid.push(cell_equilibrium);
        }
        
        let center_x = width / 2;
        let center_y = height / 2;
        let center_index = center_y * width + center_x;
        for i in 0..lattice::Q {
            grid[center_index][i] *= 1.1;
        }
        Self {
            width,
            height,
            omega: 1.0,
            grid,
        }
    }
    
    pub fn step(&mut self) {
        let mut grid_new = Vec::with_capacity(self.grid.len());
        
        for cell in &self.grid {
            let collided = collision::collide_cell(cell, self.omega);
            grid_new.push(collided);
        }
        
        self.grid = streaming::stream_periodic(&grid_new, self.width, self.height);
    }
    
    pub fn total_mass(&self) -> f64 {
        self.grid
            .iter()
            .map(|cell| cell.iter().sum::<f64>())
            .sum()
    }
    
    pub fn boundaries(&self) -> (i32, i32) {
        (self.width as i32, self.height as i32)
    }
    
    pub fn index_from_xy(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
    
    pub fn density_at(&self, x: usize, y: usize) -> f64 {
        let idx = self.index_from_xy(x, y);
        self.grid[idx].iter().sum()
    }

}