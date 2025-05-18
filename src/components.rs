use crossterm::style::Color;

pub type Entity = usize;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32
}
impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct Velocity {
    pub dx: i32,
    pub dy: i32
}
impl Velocity {
    pub fn new(dx: i32, dy: i32) -> Self {
        Self { dx, dy }
    }
}

pub struct Collider {
    pub width: u32,
    pub height: u32   
}
impl Collider {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

pub struct Renderable {
    pub width: u32,
    pub height: u32,
    pub symbol: char,
    pub color: Color
}
impl Renderable {
    pub fn new(width: u32, height: u32, symbol: char, color: Color) -> Self {
        Self { width, height, symbol, color }
    }
}

pub enum Direction { Left, Right, Up, Down }

#[derive(Debug)]
pub struct Follows(pub Entity);

pub struct Controllable;

pub struct Growing;

pub struct Edible;