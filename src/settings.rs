pub const WINDOW_WIDTH: u32 = 600;
pub const WINDOW_TITLE: &str = "The Great Sugar Empire";
pub const ENTITY_TOTAL_NUMBER: usize = 100;
pub const WORLD_GRID_NUMBER: usize = 20;
pub const SUGAR_RADIUS_IN_COOR: f64 = (WORLD_GRID_NUMBER >> 1) as f64;
pub const SUGAR_MAX_PORDUCTION: f64 = 50.0;
pub const WORLD_GIRD_WIDTH: u32 = WINDOW_WIDTH / WORLD_GRID_NUMBER as u32;
pub const ENTITY_DRAW_RADIUS: i16 = (WORLD_GIRD_WIDTH as f64 / 4.0) as i16;
pub const ENTITY_SUGAR_CONSUMED: f64 = 10.0;
pub const ENTITY_VISION_DISTANCE: i16 = 2;
pub const TARGET_RED: f64 = 220.0;
pub const TARGET_GREEN: f64 = 160.0;
pub const TARGET_BLUE: f64 = 190.0;
pub const ENTITY_DRAW_COLOR_RED: u8 = 0;
pub const ENTITY_DRAW_COLOR_GREEN: u8 = 0;
pub const ENTITY_DRAW_COLOR_BLUE: u8 = 120;
pub const DAYTIME_SECOND: f64 = 2.0;
pub const CONSIDERATION_SECOND: f64 = 0.2 * DAYTIME_SECOND;
pub const MOVING_SECOND: f64 = 0.6 * DAYTIME_SECOND;
