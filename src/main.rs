
mod sim;
use sim::Simulation;

fn main() {
    let width = 16;
    let height = 16;
    let mut sim = Simulation::new(16, 16);
    for step in 0..10 {
        sim.step();
        println!("stepped {step} and mass is {}", sim.total_mass());
        for y in 0..height  {
            for x in 0..width {
                print!("{}", sim.density_at(x, y) as i32);
            }
            println!();
        }
    }
    println!("boundary limits are: {:?}", sim.boundaries());
}