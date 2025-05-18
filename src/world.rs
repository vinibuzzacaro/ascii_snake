use std::collections::{HashMap, HashSet};

use crossterm::style::Color;
use rand::{rngs::ThreadRng, Rng};

use crate::components::{Collider, Entity, Follows, Position, Renderable, Velocity};

pub struct World {
    next_entity: Entity,
    rng: ThreadRng,
    pub field_width: u32,
    pub field_height: u32,
    pub entities: HashSet<Entity>,
    pub positions: HashMap<Entity, Position>,
    pub velocities: HashMap<Entity, Velocity>,
    pub followers: HashMap<Entity, Follows>,
    pub controllables: HashSet<Entity>,
    pub growing: HashSet<Entity>,
    pub edibles: HashSet<Entity>,
    pub colliders: HashMap<Entity, Collider>,
    pub renderables: HashMap<Entity, Renderable>
}
impl World {
    pub fn new(field_width: u32, field_height: u32) -> Self {
        Self { 
            next_entity: 0, 
            rng: rand::rng(),
            field_width, 
            field_height, 
            entities: HashSet::new(), 
            positions: HashMap::new(), 
            velocities: HashMap::new(), 
            followers: HashMap::new(), 
            controllables: HashSet::new(), 
            growing: HashSet::new(), 
            edibles: HashSet::new(), 
            colliders: HashMap::new(), 
            renderables: HashMap::new() 
        }
    }

    fn get_next_entity(&mut self) -> Entity {
        self.next_entity += 1;
        self.next_entity - 1
    }

    pub fn create_entity(&mut self) -> Entity {
        let e = self.get_next_entity();
        self.entities.insert(e);
        e
    }

    pub fn remove_entity(&mut self, e: Entity) {
        self.entities.remove(&e);
        self.positions.remove(&e);
        self.velocities.remove(&e);
        self.followers.remove(&e);
        self.controllables.remove(&e);
        self.growing.remove(&e);
        self.edibles.remove(&e);
        self.colliders.remove(&e);
        self.renderables.remove(&e);
    }

    pub fn spawn_head(&mut self) -> Entity {
        let head = self.create_entity();
        self.velocities.insert(head, Velocity::new(0, 1));
        self.controllables.insert(head);
        self.positions.insert(head, Position::new(2, 3));
        self.colliders.insert(head, Collider::new(1, 1));
        self.renderables.insert(head, Renderable::new(1, 1, '%', Color::DarkGreen));        
        head
    }

    pub fn spawn_follower(&mut self) -> Entity {
        let e = self.create_entity();
        let leading = self.followers
        .iter()
        .fold(0, |max, (e, _)| 
            if max > *e { max } else { *e }
        );
        self.followers.insert(e, Follows(leading));
        self.colliders.insert(e, Collider::new(1, 1));
        self.renderables.insert(e, Renderable::new(1, 1, '+', Color::Green));
        if let Some(leading_pos) = self.positions.get(&leading) {
            self.positions.insert(e, Position::new(leading_pos.x - 1, leading_pos.y - 1));
        }
        e
    }

    pub fn spawn_food(&mut self) {
        let food = self.create_entity();    
        let mut occupied_positions = self.positions
            .iter()
            .filter_map(|(e, pos)| {
                if self.controllables.get(e).is_some() || self.followers.get(e).is_some() {
                    Some(pos.clone())
                } else {
                    None
                }
            });
        if occupied_positions.clone().count() == (self.field_height * self.field_width) as usize {
            return;
        }
        let position = loop {
            let x = self.rng.random_range(1..self.field_width - 1) as i32;
            let y = self.rng.random_range(1..self.field_height - 1) as i32;
            let position = Position::new(x, y);
            if !occupied_positions.any(|pos| pos == position) {                
                break position
            }
        };    
        self.positions.insert(food, position);
        self.edibles.insert(food);
        self.renderables.insert(food, Renderable::new(1, 1, '@', Color::Red));
    }
}