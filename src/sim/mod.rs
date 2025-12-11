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

mod collision;
mod lattice;
mod streaming;

pub struct Simulation {
    width: usize,
    height: usize,
    omega: f64, // BGK relaxation rate which controls how quickly equilibrium affects it
    grid: Vec<[f64; lattice::Q]>, // "fluid" cells which have density and direction values
    solid: Vec<bool>,             // walls, the car, etc.
}

/// Creates a new `Simulation` instance
///
/// # Fields:
/// width: how wide the grid is
/// height: how tall the grid is
///
/// # Returns
/// A new simulation with set density initialized in all cells
///
impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        let initial_density = 1.0;
        let initial_velocity_x = 0.11; // Used to give the flow a bit of drift one direction
        let initial_velocity_y = 0.0;

        // calculates where the cells are relaxing towards based on density and velocity
        let cell_equilibrium = lattice::equilibrium_calculator(
            initial_density,
            initial_velocity_x,
            initial_velocity_y,
        );

        // the simulation size
        let size = width * height;
        let mut grid = Vec::with_capacity(size); // a 1D representation of 2D grid
        for _ in 0..size {
            grid.push(cell_equilibrium);
        }

        let mut solid = vec![false; size];

        // builds the "car" object (which looks awful... I know)
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;

                // normalizes the coordinates for 2D -> 1D translation
                let nx = x as f64 / (width as f64);
                let ny = y as f64 / (height as f64);

                // sets the "car" cells as solid
                if car_shape_normalized(nx, ny) {
                    solid[idx] = true;
                    
                    grid[idx] = [0.0; lattice::Q];
                }
            }
        }

        Self {
            width,
            height,
            omega: 1.0,
            grid,
            solid,
        }
    }

    /// Builds inlet and outlet
    ///
    /// Left boundary implements a density and velocity
    /// Right boundary copies from neighbor
    fn apply_inlet_outlet(&mut self) {
        let inlet_density = 0.9;
        let inlet_velocity_x = 0.1;
        let inlet_velocity_y = 0.0;

        // Left boundary: determines equilibrium 
        for y in 0..self.height {
            let idx = self.index_from_xy(0, y); // only affects the leftmost cells
            if self.solid[idx] { 
                continue;
            }
            self.grid[idx] =
                lattice::equilibrium_calculator(inlet_density, inlet_velocity_x, inlet_velocity_y);
        }

        // Right boundary: outflow simply copies its neighbor data
        // keeps it from bouncing or wrapping
        if self.width >= 2 {
            for y in 0..self.height {
                let idx_out = self.index_from_xy(self.width - 1, y);
                let idx_in = self.index_from_xy(self.width - 2, y);
                if self.solid[idx_out] {
                    continue;
                }
                self.grid[idx_out] = self.grid[idx_in];
            }
        }
    }

    /// Runs the actual calculations in a step of the simulation
    pub fn step(&mut self) {
        let mut grid_new = Vec::with_capacity(self.grid.len()); // post-compute grid

        for cell in &self.grid {
            let collided = collision::collide_cell(cell, self.omega); // collision checks
            grid_new.push(collided); // adds new cell to post-compute grid
        }

        // passes in sim details
        self.grid = streaming::stream_periodic(&grid_new, &self.solid, self.width, self.height);
        self.apply_inlet_outlet();
    }

    /// this is used to abstract the funky 1D -> 2D math
    pub fn index_from_xy(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// finds the density in a cell using the 2D indices
    pub fn density_at(&self, x: usize, y: usize) -> f64 {
        let idx = self.index_from_xy(x, y);
        self.grid[idx].iter().sum()
    }

    /// check if it's a solid cell which behaves differently
    pub fn is_solid(&self, x: usize, y: usize) -> bool {
        let idx = self.index_from_xy(x, y);
        self.solid[idx]
    }
}


/// this was a pain in the ass
fn car_shape_normalized(nx: f64, ny: f64) -> bool {
    // Rectangle aka the "body" of the car
    let body_left = 0.3;
    let body_right = 0.7;
    let body_top = 0.70;
    let body_bottom = 0.78;

    // uses the x and y restrictions of if it's within the "car"
    let in_body = nx >= body_left && nx <= body_right && ny >= body_top && ny <= body_bottom;

    // Semicircle aka the "roof" of the car
    let roof_center_x = 0.6;
    let roof_center_y = body_top;
    let roof_radius = 0.08;
    let dx = nx - roof_center_x;
    let dy = ny - roof_center_y;
    let in_roof_circle = dx * dx + dy * dy <= roof_radius * roof_radius;

    // uses the x and y restrictions of if it's within the "car"
    let in_roof = in_roof_circle && ny <= roof_center_y;

    // returns true if it's "in the car" so it can be marked solid 
    in_body || in_roof
}
