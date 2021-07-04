use crate::settings::WORLD_GRID_NUMBER;

pub struct WorldGrid {
    grid_matrix: [[u32; WORLD_GRID_NUMBER]; WORLD_GRID_NUMBER],
}

impl WorldGrid {
    pub fn new() -> Self {
        WorldGrid {
            grid_matrix: [[0; WORLD_GRID_NUMBER]; WORLD_GRID_NUMBER],
        }
    }
}
