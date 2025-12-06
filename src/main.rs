
mod sim;
use sim::Simulation;

fn main() {
    let mut sim = Simulation::new(16, 16);
    let cell = [1.0f64; lattice::Q];
    let result = collision::collide_cell(&cell, 1.0);
    println!("{:?}", &result[0..3]);
}