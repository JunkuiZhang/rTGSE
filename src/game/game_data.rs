pub enum GameStatus {
    Running,
    Paused,
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
