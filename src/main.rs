
mod sim;
use sim::Simulation;

fn main() {
    let mut sim = Simulation::new(16, 16);
    for step in 0..10 {
        sim.step();
        println!("stepped {step} and mass is {}", sim.total_mass());
    }
    println!("boundary limits are: {:?}", sim.boundaries());
}