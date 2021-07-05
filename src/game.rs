use std::time::SystemTime;

use crate::{
    settings::{
        ENTITY_COLOR, ENTITY_COLOR_ORIGIN, ENTITY_DRAW_RADIUS, ENTITY_TOTAL_NUMBER, LINE_COLOR,
        SUGAR_MAX_PORDUCTION, WORLD_GIRD_WIDTH, WORLD_GRID_NUMBER,
    },
    world_grid::WorldGrid,
};

use self::{
    game_data::{DayTimeStatus, EntityStatus, GameStatus, GridCoordinate},
    game_system::{
        color_interpolate, day_time_check, entity_consideration_update, entity_daytime_update,
        entity_moving_update,
    },
};
use rand::Rng;
use sdl2::{
    event::Event, gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color, rect::Rect,
    render::WindowCanvas, EventPump,
};

pub mod game_data;
mod game_system;

pub struct Game {
    // Game Info
    game_running: bool,
    game_status: GameStatus,
    time_stamp: SystemTime,
    day_count: u32,
    day_count_indicator: bool,

    // Game Systems
    event_pump: EventPump,
    canvas: WindowCanvas,

    // Game Data
    world_grid: WorldGrid,
    entity_status_list: [EntityStatus; ENTITY_TOTAL_NUMBER],
    entity_pos_list: [GridCoordinate; ENTITY_TOTAL_NUMBER],
    entity_target_pos_list: [GridCoordinate; ENTITY_TOTAL_NUMBER],
}

impl Game {
    pub fn new(width: u32, title: &str) -> Self {
        let game_running = true;
        let game_status = GameStatus::Paused;
        let mut rng = rand::thread_rng();
        let time_stamp = SystemTime::now();

        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let game_window = video_subsys
            .window(title, width, width)
            .position_centered()
            .build()
            .unwrap();
        let canvas = game_window.into_canvas().build().unwrap();

        let mut world_grid = WorldGrid::new();
        world_grid.matrix_gen();

        let entity_status_list = [EntityStatus {
            is_alive: true,
            current_wealth: 10.0,
            is_calculated: false,
        }; ENTITY_TOTAL_NUMBER];

        let mut entity_pos_list = [GridCoordinate::new(0, 0); ENTITY_TOTAL_NUMBER];
        for i in 0..ENTITY_TOTAL_NUMBER {
            let mut coor = GridCoordinate::new(
                rng.gen_range(0..WORLD_GRID_NUMBER),
                rng.gen_range(0..WORLD_GRID_NUMBER),
            );
            while entity_pos_list.contains(&coor) {
                coor = GridCoordinate::new(
                    rng.gen_range(0..WORLD_GRID_NUMBER),
                    rng.gen_range(0..WORLD_GRID_NUMBER),
                );
            }
            // println!("{:?}", coor);
            entity_pos_list[i] = coor;
        }
        let entity_target_pos_list = entity_pos_list.clone();

        Game {
            game_running,
            game_status,
            time_stamp,
            day_count: 0,
            day_count_indicator: false,
            event_pump,
            canvas,
            world_grid,
            entity_status_list,
            entity_pos_list,
            entity_target_pos_list,
        }
    }

    fn events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.game_running = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    self.game_status.toggle();
                }
                _ => {}
            }
        }
    }

    pub fn run(&mut self) {
        self.time_stamp = SystemTime::now();
        while self.game_running {
            self.events();
            self.update_and_render();
        }
    }

    fn update_and_render(&mut self) {
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();
        match self.game_status {
            GameStatus::Running => {
                self.grid_render();
                let daytime_status = day_time_check(self.time_stamp);
                self.entities_behaviours(daytime_status);
            }
            GameStatus::Paused => {
                self.grid_render();
                self.paused_draw();
            }
        }
        self.canvas.present();
    }

    fn grid_render(&mut self) {
        for row in 0..WORLD_GRID_NUMBER {
            for col in 0..WORLD_GRID_NUMBER {
                let x = row as i32 * WORLD_GIRD_WIDTH as i32;
                let y = col as i32 * WORLD_GIRD_WIDTH as i32;
                let rect = Rect::new(x, y, WORLD_GIRD_WIDTH, WORLD_GIRD_WIDTH);
                let portion = self.world_grid.grid_matrix[row][col] / SUGAR_MAX_PORDUCTION;
                self.canvas.set_draw_color(color_interpolate(portion));
                self.canvas.fill_rect(rect).unwrap();
            }
        }
    }

    fn entities_behaviours(&mut self, daytime_status: DayTimeStatus) {
        for i in 0..ENTITY_TOTAL_NUMBER {
            let status = self.entity_status_list[i].clone();
            if !status.is_alive {
                continue;
            }
            let status = &mut self.entity_status_list[i];

            match daytime_status {
                DayTimeStatus::DayTimeArrived => {
                    self.entity_pos_list = self.entity_target_pos_list.clone();
                    let pos = &self.entity_pos_list[i];
                    entity_daytime_update(
                        pos,
                        status,
                        &self.world_grid.grid_matrix,
                        &mut self.day_count,
                        &mut self.day_count_indicator,
                        &mut self.time_stamp,
                    );
                    self.entity_draw(pos)
                }
                // Also consideration period
                DayTimeStatus::MovementCompleted => {
                    let pos = &self.entity_pos_list[i];
                    entity_consideration_update(
                        i,
                        status,
                        &self.entity_pos_list,
                        &mut self.entity_target_pos_list,
                        &self.world_grid.grid_matrix,
                        &mut self.day_count_indicator,
                    );
                    self.entity_draw(pos);
                }
                DayTimeStatus::EntitiesMoving(time_period) => {
                    let pos = self.entity_pos_list[i].clone();
                    let des = self.entity_target_pos_list[i].clone();
                    let (sx, sy) = pos.entity_to_draw();
                    let (dx, dy) = entity_moving_update(&pos, &des, time_period);
                    self.moving_line_draw(sx, sy, dx, dy);
                    self.entity_draw_raw(sx, sy, true);
                    self.entity_draw_raw(dx, dy, false);
                }
            }
        }
    }

    fn paused_draw(&self) {
        for i in 0..ENTITY_TOTAL_NUMBER {
            let status = &self.entity_status_list[i];
            if !status.is_alive {
                continue;
            }
            let pos = &self.entity_pos_list[i];
            self.entity_draw(pos);
        }
    }

    fn entity_draw(&self, pos: &GridCoordinate) {
        let (x, y) = pos.entity_to_draw();
        self.entity_draw_raw(x, y, false);
    }

    fn entity_draw_raw(&self, x: i16, y: i16, moving_indicator: bool) {
        let color;
        if moving_indicator {
            color = ENTITY_COLOR_ORIGIN;
        } else {
            color = ENTITY_COLOR;
        }
        self.canvas
            .filled_circle(x, y, ENTITY_DRAW_RADIUS, color)
            .unwrap();
    }

    fn moving_line_draw(&self, sx: i16, sy: i16, dx: i16, dy: i16) {
        self.canvas
            .thick_line(sx, sy, dx, dy, 2, LINE_COLOR)
            .unwrap();
    }
}
