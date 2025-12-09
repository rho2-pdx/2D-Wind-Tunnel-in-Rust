//! Moves cells around after collisions computed


use super::lattice;

pub fn stream_periodic(
    grid_old: &[[f64; lattice::Q]],
    solid: &[bool],
    width: usize,
    height: usize,
) -> Vec<[f64; lattice::Q]> {
    let mut grid_new = vec![[0.0f64; lattice::Q]; grid_old.len()];
    let w = width as i32;
    let h = height as i32;
    
    let grid_index = |x: i32, y: i32| -> usize {
        (y as usize) * width + (x as usize)
    };
    
    for y in 0..h {
        for x in 0..w {
            let src_index = grid_index(x, y);

            // Solid cells act only as boundaries; they do not stream their own fluid.
            if solid[src_index] {
                continue;
            }

            let cell = grid_old[src_index];

            for (i, direction) in lattice::DIRECTIONS.iter().enumerate() {
                let (dx, dy) = lattice::direction_vector(*direction);

                // Horizontal: no wrap-around. If we step out of bounds, let
                // inlet/outlet boundary conditions handle it.
                let nx = x + dx;
                if nx < 0 || nx >= w {
                    continue;
                }

                // Vertical: periodic wrap.
                let ny = (y + dy + h) % h;

                let dst_index = grid_index(nx, ny);

                if solid[dst_index] {
                    // Bounce-back: reflect this population back into the source cell.
                    let bounce_back = lattice::OPPOSITE_DIRECTION_INDEX[i];
                    grid_new[src_index][bounce_back] += cell[i];
                } else {
                    // Normal streaming into neighbor cell.
                    grid_new[dst_index][i] += cell[i];
                }
            }
        }
    }
    grid_new
}