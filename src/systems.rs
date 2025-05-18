use core::panic;
use std::{cmp::{max, min}, collections::{HashMap, HashSet}, time::Duration};

use crossterm::event::{self, poll, Event, KeyCode};

use crate::{components::{Entity, Follows, Position}, game::{Game, GameState}, world::World};

pub struct MovementSystem;
impl MovementSystem {
    pub fn run(world: &mut World) {        
        Self::follow_segments(world);
        Self::move_head(world);
    }

    fn move_head(world: &mut World) {
        for e in &world.controllables {
            if let (Some(vel), Some(pos)) = (
                world.velocities.get(e),
                world.positions.get_mut(e)
            ) {
                pos.x += vel.dx;
                pos.y += vel.dy;
            }
        }        
    }

    fn follow_segments(world: &mut World) {
        let leading_entities_positions: HashMap<Entity, Position> = world.followers
            .iter()
            .filter_map(|(e, Follows(f))| {
                if let Some(pos) = world.positions.get(f) {
                    Some((*e, pos.clone()))
                } else {
                    None
                }
            })
            .collect();
        for (e, _) in &world.followers {
            if let (Some(e_pos), Some(leading_pos)) = (
                world.positions.get_mut(e),
                leading_entities_positions.get(e)
            ) {
                *e_pos = leading_pos.clone();
            }
        }
    }
}

pub struct EatingSystem;
impl EatingSystem {
    pub fn run(game: &mut Game) {
        let contacts = Self::detect_food_contact(&mut game.world);
        for (head, food) in contacts {
            game.world.growing.insert(head);
            game.world.remove_entity(food);
            game.world.spawn_food();
        }
        Self::process_growth(game);
    }

    fn process_growth(game: &mut Game) {
        let entities_to_grow: Vec<Entity> = game.world.growing.iter().copied().collect();
        for e in entities_to_grow {
            game.world.spawn_follower();
            game.world.growing.remove(&e);
            game.score += 1;
        }
    }

    fn detect_food_contact(world: &World) -> Vec<(Entity, Entity)> {        
        let mut contacts = vec![];
        for head in &world.controllables {
            for food in &world.edibles {
                if world.positions.get(head) == world.positions.get(food) {
                    contacts.push((*head, *food));
                }
            }
        }
        contacts
    }
}

pub struct CollisionSystem;
impl CollisionSystem {
    fn detect_wall_collision(world: &World) -> Vec<Entity> {
        world.positions
            .iter()
            .filter_map(|(e, p)| {
                if p.x < 0 || p.y < 0 || p.x > (world.field_width - 1) as i32 || p.y > (world.field_height - 1) as i32 {
                    Some(*e)
                } else {
                    None
                }                
            })
            .collect::<Vec<Entity>>()
    }

    fn detect_segment_collision(world: &World) -> bool {
        let segment_count = world.followers
            .iter()
            .count();
        let distinct_positions = world.followers
            .iter()
            .filter_map(|(e, _)| world.positions.get(e))
            .collect::<HashSet<&Position>>()
            .len();
        segment_count != distinct_positions
    }

    pub fn run(world: &mut World) -> GameState {
        if Self::detect_segment_collision(world) {
            return GameState::GameOver
        }
        let last_row = world.field_height - 1;
        let last_column = world.field_width - 1;
        for e in Self::detect_wall_collision(&world) {                        
            if let Some(pos) = world.positions.get_mut(&e) {
                if pos.x < 0 {
                    pos.x = last_column as i32
                } else if pos.x > last_column as i32 {
                    pos.x = 0
                }
                if pos.y < 0 {
                    pos.y = last_row as i32
                } else if pos.y > last_row as i32 {
                    pos.y = 0
                }
            }
        }
        GameState::Playing
    }
}

pub struct InputSystem;
impl InputSystem {
    pub fn run(world: &mut World) {        
        if let Ok(true) = poll(Duration::from_secs(0)) {
            if let Ok(Event::Key(event)) = event::read() {                
                for id in &world.controllables {
                    if let Some(vel) = world.velocities.get_mut(id) {
                        match event.code {
                            KeyCode::Char('w') | KeyCode::Char('W') => {
                                vel.dy = -1;
                                vel.dx = 0;
                            },
                            KeyCode::Char('a') | KeyCode::Char('A') => {
                                vel.dx = -1;
                                vel.dy = 0;
                            },
                            KeyCode::Char('s') | KeyCode::Char('S') => {
                                vel.dy = 1;
                                vel.dx = 0;
                            },
                            KeyCode::Char('d') | KeyCode::Char('D') => {
                                vel.dx = 1;
                                vel.dy = 0;
                            }, 
                            KeyCode::Esc => std::process::exit(0),                           
                            _ => return
                        }
                    }
                }
            }
        }
    }
}