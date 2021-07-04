use crate::world_grid::WorldGrid;

use self::game_data::GameStatus;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::WindowCanvas, EventPump};

mod game_data;

pub struct Game {
    // Game Info
    game_running: bool,
    game_status: GameStatus,

    // Game Systems
    event_pump: EventPump,
    canvas: WindowCanvas,
    // Game Data
    world_grid: WorldGrid,
}

impl Game {
    pub fn new(width: u32, title: &str) -> Self {
        let game_running = true;
        let game_status = GameStatus::Paused;

        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let video_subsys = sdl_context.video().unwrap();
        let game_window = video_subsys
            .window(title, width, width)
            .position_centered()
            .build()
            .unwrap();
        let canvas = game_window.into_canvas().build().unwrap();

        let world_grid = WorldGrid::new();

        Game {
            game_running,
            game_status,
            event_pump,
            canvas,
            world_grid,
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
                println!("Game running...");
            }
            GameStatus::Paused => {
                println!("Game has been paused...");
            }
        }
        self.canvas.present();
    }
}
