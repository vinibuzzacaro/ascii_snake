use std::{io::{self, stdout, Stdout}, thread, time::{Duration, Instant}};

use crossterm::{cursor, execute, terminal::{self, disable_raw_mode, enable_raw_mode, Clear}};

use crate::{renderer::Renderer, systems::{CollisionSystem, EatingSystem, InputSystem, MovementSystem}, world::World};

#[derive(PartialEq)]
pub enum GameState {
    Playing, 
    Paused,
    GameOver
}

pub struct Game {
    pub world: World,
    renderer: Renderer,
    pub score: u16
}
impl Game {
    pub fn new(field_width: u32, field_height: u32) -> Self {
        Game { renderer: Renderer::default(),score: 0, world: World::new(field_width, field_height) }
    }

    fn update(&mut self) {
        InputSystem::run(&mut self.world);
        MovementSystem::run(&mut self.world);
        if CollisionSystem::run(&mut self.world) == GameState::GameOver {
            self.renderer.game_over_screen(self.score);
            std::process::exit(0);
        };
        EatingSystem::run(self);        
    }

    fn render(&mut self) -> io::Result<()> {        
        self.renderer.run(&self.world, self.score)
    }
    
    fn initialize(&mut self) {
        if let Err(e) = self.renderer.initialize() {
            eprintln!("Failed to initialize renderer: {e}");
            std::process::exit(1);
        };
        let _head = self.world.spawn_head();
        let _segment = self.world.spawn_follower();
        self.world.spawn_food();
    }

    pub fn run(&mut self) -> io::Result<()> {
        const TARGET_FPS: f32 = 8.0;        
        let frame_duration = Duration::from_secs_f32(1.0 / TARGET_FPS);
        let mut frame_start: Instant;
        let mut elapsed: Duration;        
        execute!(self.renderer.stdout, cursor::Hide)?; 
        enable_raw_mode()?;       
        self.initialize();
        self.render()?;        
        loop {
            frame_start = Instant::now();
            self.update();
            self.render()?;            
            elapsed = frame_start.elapsed();
            if elapsed < frame_duration {
                thread::sleep(frame_duration - elapsed);
            }
        }        
    }
}