//! Handles the cellular collision math
//!
//! Gets the f value (the calculated cardinal vector data) as "f_old" and outputs "f_new"
//! based upon this
use super::lattice;

/// Takes in the cell data and omega (which controls the equilibrium draw)
/// outputs new cell data
/// Uses the fluid calculator and equilibrium calculator, and is used in Simulation::step
pub fn collide_cell(f_old: &[f64; lattice::Q], omega: f64) -> [f64; lattice::Q] {
    let (density, velocity_x, velocity_y) = lattice::fluid_calculator(f_old);
    let equilibrium = lattice::equilibrium_calculator(density, velocity_x, velocity_y);

    let mut f_new = [0.0f64; lattice::Q];
    for i in 0..lattice::Q {
        f_new[i] = f_old[i] + omega * (equilibrium[i] - f_old[i]);
    }
    f_new
}
