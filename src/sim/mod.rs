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
    grid: Vec<[f64; lattice::Q]>, // "fluid" cells which have density and direction values
    solid: Vec<bool>,             // walls, the car, etc.
    smoke: Vec<f64>,              // passive tracer used only for visualization
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
        let initial_velocity_x = 0.05;
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
        let mut solid = vec![false; size];


        let car_width = width / 5;        // 1/5 of domain width
        let car_height = height / 6;      // 1/6 of domain height
        let car_center_x = width / 2;
        let car_base_y = (height * 2) / 3; // lower third of domain

        let car_min_x = car_center_x.saturating_sub(car_width / 2);
        let car_max_x = (car_center_x + car_width / 2).min(width - 1);
        let car_min_y = car_base_y.saturating_sub(car_height);
        let car_max_y = car_base_y.min(height - 2);

        for y in car_min_y..=car_max_y {
            for x in car_min_x..=car_max_x {
                let idx = y * width + x;
                solid[idx] = true;
                // Solid cells do not contain fluid; clear their distributions.
                grid[idx] = [0.0; lattice::Q];
            }
        }

        // Initialize a separate smoke tracer field with no smoke initially.
        let smoke = vec![0.0; size];

        Self {
            width,
            height,
            omega: 1.0,
            grid,
            solid,
            smoke,
        }
    }

    /// Apply simple inlet (left) and outlet (right) boundary conditions.
    ///
    /// Left boundary: impose a fixed density and rightward velocity.
    /// Right boundary: copy from the neighboring interior column (simple outflow).
    fn apply_inlet_outlet(&mut self) {
        let inlet_density = 1.0;
        let inlet_velocity_x = 0.05;
        let inlet_velocity_y = 0.0;

        // Left boundary: impose equilibrium with fixed density and velocity.
        for y in 0..self.height {
            let idx = self.index_from_xy(0, y);
            if self.solid[idx] {
                continue;
            }
            self.grid[idx] = lattice::equilibrium_calculator(
                inlet_density,
                inlet_velocity_x,
                inlet_velocity_y,
            );
        }

        // Right boundary: simple outflow - copy from neighbor column.
        if self.width >= 2 {
            for y in 0..self.height {
                let idx_out = self.index_from_xy(self.width - 1, y);
                let idx_in  = self.index_from_xy(self.width - 2, y);
                if self.solid[idx_out] {
                    continue;
                }
                self.grid[idx_out] = self.grid[idx_in];
            }
        }
    }

    /// Returns the smoke tracer concentration at the given cell.
    /// 0.0 means no smoke, 1.0 is very dense.
    pub fn smoke_at(&self, x: usize, y: usize) -> f64 {
        let idx = self.index_from_xy(x, y);
        self.smoke[idx]
    }

    /// Very simple advection of a passive smoke tracer from left to right.
    ///
    /// This is not a full physical advection scheme; it's a cheap way to produce
    /// a clear "smoke stream" flowing over the car while the LBM handles the
    /// underlying fluid dynamics.
    fn update_smoke(&mut self) {
        let width = self.width;
        let height = self.height;
        let mut new_smoke = vec![0.0; self.smoke.len()];

        // Define a thin band in the vertical direction where we inject smoke at the inlet.
        // This creates a more "wind tunnel" style smoke line instead of a big grey block.
        let center_y = height / 2;
        let band_half_thickness = 2; // total band ~5 cells tall
        let band_top = center_y.saturating_sub(band_half_thickness);
        let band_bottom = (center_y + band_half_thickness).min(height.saturating_sub(1));

        for y in 0..height {
            for x in 0..width {
                let idx = self.index_from_xy(x, y);
                if self.solid[idx] {
                    // No smoke inside solid cells.
                    continue;
                }

                if x == 0 {
                    // Inlet: inject fresh smoke only in the central band.
                    if y >= band_top && y <= band_bottom {
                        new_smoke[idx] = 1.0;
                    } else {
                        new_smoke[idx] = 0.0;
                    }
                } else {
                    // For interior cells, simply shift smoke from the left neighbor.
                    let left_idx = self.index_from_xy(x - 1, y);
                    if !self.solid[left_idx] {
                        new_smoke[idx] = self.smoke[left_idx];
                    } else {
                        new_smoke[idx] = 0.0;
                    }
                }
            }
        }

        // Apply a small decay so smoke gradually fades downstream.
        for value in &mut new_smoke {
            *value *= 0.98;
        }

        self.smoke = new_smoke;
    }

    pub fn step(&mut self) {
        let mut grid_new = Vec::with_capacity(self.grid.len());

        for cell in &self.grid {
            let collided = collision::collide_cell(cell, self.omega);
            grid_new.push(collided);
        }

        self.grid = streaming::stream_periodic(
            &grid_new,
            &self.solid,
            self.width,
            self.height,
        );
        self.apply_inlet_outlet();

        // Update the passive smoke tracer after advancing the fluid.
        self.update_smoke();
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

    pub fn is_solid(&self, x: usize, y: usize) -> bool {
        let idx = self.index_from_xy(x, y);
        self.solid[idx]
    }

}