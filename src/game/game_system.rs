use std::time::{SystemTime};

use sdl2::pixels::Color;

use crate::settings::{CONSIDERATION_SECOND, DAYTIME_SECOND, ENTITY_SUGAR_CONSUMED, ENTITY_TOTAL_NUMBER, ENTITY_VISION_DISTANCE, MOVING_SECOND, TARGET_BLUE, TARGET_GREEN, TARGET_RED, WORLD_GRID_NUMBER};

use super::game_data::{DayTimeStatus, EntityStatus, GridCoordinate};

pub fn entity_daytime_update(
    pos: &GridCoordinate,
    status: &mut EntityStatus,
    grid: &[[f64; WORLD_GRID_NUMBER]; WORLD_GRID_NUMBER],
    daytime_count: &mut u32,
    daytime_count_indicator: &mut bool,
    time_stamp: &mut SystemTime
) {
    if !(*daytime_count_indicator) {
        *daytime_count_indicator = true;
        *daytime_count += 1;
        *time_stamp = SystemTime::now();
    }
    if status.is_calculated {
        return;
    }
    let income = grid[pos.row][pos.col];
    status.current_wealth += income - ENTITY_SUGAR_CONSUMED;
    if status.current_wealth < 0.0 {
        status.is_alive = false;
    }
    status.is_calculated = true;
}

pub fn entity_moving_update(
    pos: &GridCoordinate,
    target_pos: &GridCoordinate,
    time_period: f64,
) -> (i16, i16) {
    let (start_x, start_y) = pos.entity_to_draw();
    let (dest_x, dest_y) = target_pos.entity_to_draw();
    let portion = (time_period - CONSIDERATION_SECOND) / MOVING_SECOND;
    if portion >= 1.0 {
        return (dest_x, dest_y);
    }
    let res_x = (start_x as f64 + (dest_x as f64 - start_x as f64) * portion).round() as i16;
    let res_y = (start_y as f64 + (dest_y as f64 - start_y as f64) * portion).round() as i16;
    return (res_x, res_y);
}

pub fn entity_consideration_update(
    id: usize,
    status: &mut EntityStatus,
    pos_list: &[GridCoordinate; ENTITY_TOTAL_NUMBER],
    target_list: &mut [GridCoordinate; ENTITY_TOTAL_NUMBER],
    grid: &[[f64; WORLD_GRID_NUMBER]; WORLD_GRID_NUMBER],
    daytime_indicator: &mut bool
) {
    if status.is_calculated {
        status.is_calculated = false;
    }
    if *daytime_indicator {
        *daytime_indicator = false;
    }
    let pos = &pos_list[id];
    let (row_start, row_end, col_start, col_end) = entity_get_vision(pos);
    let mut res = None;
    let mut base_prod = grid[pos.row][pos.col];
    // println!("{} consider range: row {}->{}, col{}->{}", id, row_start, row_end, col_start, col_end);
    for i in row_start..row_end {
        for j in col_start..col_end {
            if i == pos.row && j == pos.col {
                continue;
            }
            let coor = GridCoordinate::new(i, j);
            if pos.distance_with(&coor) > ENTITY_VISION_DISTANCE as f64 {
                continue;
            }
            if target_list.contains(&coor) {
                continue;
            }
            // println!("{} consider {:?}", id, coor);
            let target_prod = grid[i][j];
            if target_prod > base_prod {
                base_prod = target_prod;
                res = Some(coor);
            }
        }
    }
    if let Some(target) = res {
        target_list[id] = target;
        // println!("{} moving", id);
    }
}

pub fn color_interpolate(portion: f64) -> Color {
    if portion <= 0.0 {
        return Color::RGB(220, 220, 220);
    }
    let r = (230.0 * (1.0 - portion) + TARGET_RED * portion).round() as u8;
    let g = (230.0 * (1.0 - portion) + TARGET_GREEN * portion).round() as u8;
    let b = (230.0 * (1.0 - portion) + TARGET_BLUE * portion).round() as u8;
    return Color::RGB(r, g, b);
}

pub fn day_time_check(start: SystemTime) -> DayTimeStatus {
    let now = SystemTime::now();
    let duration = now.duration_since(start).unwrap().as_secs_f64();
    if duration <= CONSIDERATION_SECOND {
        return DayTimeStatus::MovementCompleted;
    } else if duration <= DAYTIME_SECOND {
        return DayTimeStatus::EntitiesMoving(duration);
    } else {
        return DayTimeStatus::DayTimeArrived;
    }
}

fn entity_get_vision(pos: &GridCoordinate) -> (usize, usize, usize, usize) {
    let mut row_start = pos.row as i16 - ENTITY_VISION_DISTANCE;
    let mut row_end = pos.row as i16 + ENTITY_VISION_DISTANCE;
    let mut col_start = pos.col as i16 - ENTITY_VISION_DISTANCE;
    let mut col_end = pos.col as i16 + ENTITY_VISION_DISTANCE;
    if row_start < 0 {
        row_start = 0;
    }
    if row_end > WORLD_GRID_NUMBER as i16 {
        row_end = WORLD_GRID_NUMBER as i16;
    }
    if col_start < 0 {
        col_start = 0;
    }
    if col_end > WORLD_GRID_NUMBER as i16 {
        col_end = WORLD_GRID_NUMBER as i16;
    }
    return (
        row_start as usize,
        row_end as usize,
        col_start as usize,
        col_end as usize,
    );
}
