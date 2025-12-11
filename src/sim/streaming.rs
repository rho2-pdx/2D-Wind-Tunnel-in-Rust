//! Moves cells around after collisions have been computed

use super::lattice;

/// Handles cell movements after collision computation
/// 
/// # Fields:
/// grid_old: the vector of vectors which contains all cell data
/// solid: a vector of bools that dictates which cells are solids
/// width: the width of the simulation
/// height: the height of the simulation
/// 
/// # Returns:
/// grid_new: updated vector of vectors with updated cell data
pub fn stream_periodic(
    grid_old: &[[f64; lattice::Q]],
    solid: &[bool],
    width: usize,
    height: usize,
) -> Vec<[f64; lattice::Q]> {
    let mut grid_new = vec![[0.0f64; lattice::Q]; grid_old.len()];
    let w = width as i32;
    let h = height as i32;

    // this equation allows us to translate between a 1D vector representation of the 2D grid
    let grid_index = |x: i32, y: i32| -> usize {(y as usize) * width + (x as usize)};

    for y in 0..h {
        for x in 0..w {
            let src_index = grid_index(x, y); // we traverse the vector with this variable

            // solid cells get skipped
            if solid[src_index] {
                continue;
            }

            let cell = grid_old[src_index];

            for (i, direction) in lattice::DIRECTIONS.iter().enumerate() {
                let (dx, dy) = lattice::direction_vector(*direction);

                // Preventing horizontal wraparound, as a wind tunnel has an inlet and outlet
                let nx = x + dx;
                if nx < 0 || nx >= w {
                    continue;
                }

                // Vertical is just allowed for now
                let ny = (y + dy + h) % h;
                let dst_index = grid_index(nx, ny);

                // Handles bouncing from solid cells
                if solid[dst_index] {
                    let bounce_back = lattice::OPPOSITE_DIRECTION_INDEX[i];
                    grid_new[src_index][bounce_back] += cell[i];
                } else {
                    grid_new[dst_index][i] += cell[i];
                }
            }
        }
    }
    grid_new
}
