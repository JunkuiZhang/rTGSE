use crate::settings::WORLD_GIRD_WIDTH;

#[derive(Debug, Clone, Copy)]
pub enum GameStatus {
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridCoordinate {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct EntityStatus {
    pub is_alive: bool,
    pub current_wealth: f64,
    pub is_calculated: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum DayTimeStatus {
    DayTimeArrived,
    EntitiesMoving(f64),
    MovementCompleted,
}

impl GameStatus {
    pub fn toggle(&mut self) {
        match *self {
            GameStatus::Running => {
                *self = GameStatus::Paused;
            }
            GameStatus::Paused => {
                *self = GameStatus::Running;
            }
        }
    }
}

impl GridCoordinate {
    pub fn new(row: usize, col: usize) -> Self {
        return GridCoordinate { row, col };
    }

    pub fn distance_with(&self, target: &GridCoordinate) -> f64 {
        return ((self.row as f64 - target.row as f64).powi(2)
            + (self.col as f64 - target.col as f64).powi(2))
        .powf(0.5);
    }

    pub fn entity_to_draw(&self) -> (i16, i16) {
        return (
            self.row as i16 * WORLD_GIRD_WIDTH as i16 + WORLD_GIRD_WIDTH as i16 / 2,
            self.col as i16 * WORLD_GIRD_WIDTH as i16 + WORLD_GIRD_WIDTH as i16 / 2,
        );
    }
}
