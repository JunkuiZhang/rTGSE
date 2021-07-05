use crate::{
    game::game_data::GridCoordinate,
    settings::{SUGAR_MAX_PORDUCTION, SUGAR_RADIUS_IN_COOR, WORLD_GRID_NUMBER},
};

pub struct WorldGrid {
    pub grid_matrix: [[f64; WORLD_GRID_NUMBER]; WORLD_GRID_NUMBER],
}

impl WorldGrid {
    pub fn new() -> Self {
        WorldGrid {
            grid_matrix: [[0.0; WORLD_GRID_NUMBER]; WORLD_GRID_NUMBER],
        }
    }

    pub fn matrix_gen(&mut self) {
        let center_coordinate = GridCoordinate::new(WORLD_GRID_NUMBER / 2, WORLD_GRID_NUMBER / 2);
        for i in 0..WORLD_GRID_NUMBER {
            for j in 0..WORLD_GRID_NUMBER {
                let coor = GridCoordinate::new(i, j);
                let dist = coor.distance_with(&center_coordinate);
                if dist > SUGAR_RADIUS_IN_COOR {
                    continue;
                }
                self.grid_matrix[i][j] =
                    SUGAR_MAX_PORDUCTION * (1.0 - dist / SUGAR_RADIUS_IN_COOR).powi(2);
            }
        }
    }
}
