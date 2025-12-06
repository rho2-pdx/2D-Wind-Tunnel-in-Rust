//! Stores constants and attributes for the lattice

#[derive(Clone, Copy, Debug)]
pub enum CardinalDirection {
    Rest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
pub const Q: usize = 9; // number of cardinal directions from each cell

pub const DIRECTIONS: [CardinalDirection; Q] = [
    CardinalDirection::Rest,
    CardinalDirection::North,
    CardinalDirection::NorthEast,
    CardinalDirection::East,
    CardinalDirection::SouthEast,
    CardinalDirection::South,
    CardinalDirection::SouthWest,
    CardinalDirection::West,
    CardinalDirection::NorthWest,
];

pub fn direction_vector(direction: CardinalDirection) -> (i32, i32) {
    match direction {
        CardinalDirection::Rest => (0, 0),
        CardinalDirection::North => (0, 1),
        CardinalDirection::NorthEast => (1, 1),
        CardinalDirection::East => (1, 0),
        CardinalDirection::SouthEast => (1, -1),
        CardinalDirection::South => (0, -1),
        CardinalDirection::SouthWest => (-1, -1),
        CardinalDirection::West => (-1, 0),
        CardinalDirection::NorthWest => (-1, 1),
    }
}

pub fn direction_weight(direction: CardinalDirection) -> f64 {
    match direction {
        CardinalDirection::Rest          => 4.0 / 9.0,
        CardinalDirection::North 
        | CardinalDirection::East
        | CardinalDirection::South
        | CardinalDirection::West        => 1.0 / 9.0,
        CardinalDirection::NorthEast
        | CardinalDirection::SouthEast
        | CardinalDirection::NorthWest
        | CardinalDirection::SouthWest   => 1.0 / 36.0,
    }
}

/// Finds the density and velocity
pub fn fluid_calculator(f: &[f64; Q]) -> (f64, f64, f64) {
    let mut density = 0.0;
    let mut velocity_x = 0.0;
    let mut velocity_y = 0.0;
    
    for (i, direction) in DIRECTIONS.iter().enumerate() {
        let cell_amount = f[i];
        density += cell_amount;
        
        let (dx, dy) = direction_vector(*direction);
        velocity_x += cell_amount * dx as f64;
        velocity_y += cell_amount * dy as f64;
    }
    if density != 0.0 {
        velocity_x /= density;
        velocity_y /= density;
    }
    
    (density, velocity_x, velocity_y)
}

/// takes density and velocity, decides where the cell is relaxing towards
pub fn equilibrium_calculator(density: f64, velocity_x: f64, velocity_y: f64) -> [f64; Q] {
    let mut result = [0.0f64; Q];
    let squared_speed = velocity_x * velocity_x + velocity_y * velocity_y;
    
    for (i, direction) in DIRECTIONS.iter().enumerate() {
        let (dx, dy) = direction_vector(*direction);
        let direction_speed = dx as f64 * velocity_x + dy as f64 * velocity_y;
        
        let weight = direction_weight(*direction);
        
        result[i] = weight * density * 
            (1.0 + 3.0 * direction_speed + 4.5 * direction_speed * direction_speed - 1.5 * squared_speed);
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fluid_calculator_returns_zero_under_equal_opposite_forces() {
        let f = [1.0f64; Q];
        let (density, velocity_x, velocity_y) = fluid_calculator(&f);
        
        assert!((density - 9.0).abs() < 1e-10, "wrong density: {density}");
        assert!(velocity_x.abs() < 1e-10, "wrong velocity_x: {velocity_x}");
        assert!(velocity_y.abs() < 1e-10, "wrong velocity_y: {velocity_y}");
    }
}
