use std::{io::{self, stdout, Cursor, Stdout, Write}, thread::current};

use crossterm::{cursor::{self, MoveTo}, execute, style::{self, Print}, terminal, QueueableCommand};

use crate::{components::Position, game::Game, world::World};

struct RenderPosition {
    pub x: u32,
    pub y: u32
}
impl From<&Position> for RenderPosition { // In rendering, every coordinate is > 0
    fn from(value: &Position) -> Self { 
        RenderPosition { 
            x: value.x.unsigned_abs() + 1, 
            y: value.y.unsigned_abs() + 2 
        } 
    }
}

pub struct Renderer {
    pub stdout: Stdout,
    previous_frame: Option<Box<[Box<[(char, style::Color)]>]>> // (Column, Line)
}
impl Default for Renderer {
    fn default() -> Self {
        Self { stdout: stdout(), previous_frame: None }        
    }
}
impl Drop for Renderer {
    fn drop(&mut self) {
        if let Err(e) = self.shutdown() {
            eprintln!("error while dropping the renderer: {e}");
        };
    }
}
impl Renderer {    
    pub fn run(&mut self, world: &World, score: u16) -> io::Result<()> {
        const WALL_SYMBOL: char = '#';        
        const WALL_COLOR: style::Color = style::Color::White;          
        let total_columns = world.field_width + 2;
        let last_column = total_columns - 1;
        let total_rows = world.field_height + 3;        
        let last_row = total_rows - 1;
        //execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        let mut current_frame = 
            vec![
                vec![
                    (' ', style::Color::Reset); 
                    total_columns as usize
                ]
                .into_boxed_slice(); 
                total_rows as usize
            ]
            .into_boxed_slice();               
        for row in 1..total_rows {
            current_frame[row as usize][0] = (WALL_SYMBOL, WALL_COLOR); 
            current_frame[row as usize][last_column as usize] = (WALL_SYMBOL, WALL_COLOR);
            //Self::draw_char(&mut self.stdout, row, 0, WALL_SYMBOL, WALL_COLOR)?;
            //Self::draw_char(&mut self.stdout, row, last_column, WALL_SYMBOL, WALL_COLOR)?;
        }
        for column in 0..total_columns {
            current_frame[1][column as usize] = (WALL_SYMBOL, WALL_COLOR);
            current_frame[last_row as usize][column as usize] = (WALL_SYMBOL, WALL_COLOR);
            //Self::draw_char(&mut self.stdout, 1, column, WALL_SYMBOL, WALL_COLOR)?;
            //Self::draw_char(&mut self.stdout, last_row, column, WALL_SYMBOL, WALL_COLOR)?;
        }       
        for (e, ren) in &world.renderables {
            if let Some(ren_pos) = world.positions.get(e).map(|p| RenderPosition::from(p)) {
                for w in 0..ren.width {
                    let x = (ren_pos.x + w) as usize;
                    let y = ren_pos.y as usize;
                    current_frame[y][x] = (ren.symbol, ren.color);
                    //Self::draw_char(stdout, ren_pos.y, ren_pos.x + w, ren.symbol, ren.color)?;
                }
                for h in 0..ren.height {
                    let x = ren_pos.x as usize;
                    let y = (ren_pos.y + h) as usize;
                    current_frame[y][x] = (ren.symbol, ren.color);
                    //Self::draw_char(stdout, ren_pos.y + h, ren_pos.x, ren.symbol, ren.color)?;
                }
            }
        }
        if self.previous_frame.is_none() {
            self.previous_frame = Some(
                vec![
                    vec![
                        (' ', style::Color::White); 
                        total_columns as usize
                    ]
                    .into_boxed_slice(); 
                    total_rows as usize
                ]
                .into_boxed_slice()
            );
        }                    
        if let Some(prev_frame) = &self.previous_frame {
            for y in 1..total_rows as usize {
                for x in 0..total_columns as usize {                    
                    if prev_frame[y][x] != current_frame[y][x] {
                        let (symbol, color) = current_frame[y][x];
                        Self::draw_char(&mut self.stdout, y as u32, x as u32, symbol, color)?;
                    }
                }
            }
        }
        self.draw_score(score)?;
        self.previous_frame = Some(current_frame);             
        self.stdout.flush()?;
        Ok(())
    }

    fn draw_score(&mut self, score: u16) -> io::Result<()> {
        self.stdout
            .queue(cursor::MoveTo(1, 0))?
            .queue(style::SetForegroundColor(style::Color::Yellow))?
            .queue(style::Print(format!("score: {score}")))?;
        Ok(())
    }

    pub fn initialize(&mut self) -> io::Result<()> {
        execute!(
            self.stdout,
            terminal::EnterAlternateScreen,
            terminal::Clear(terminal::ClearType::All),
            cursor::Hide
        )?;
        
        // Enable raw mode to get immediate input
        terminal::enable_raw_mode()        
    }

    fn shutdown(&mut self) -> io::Result<()> {
        execute!(
            self.stdout,
            cursor::Show,
            terminal::LeaveAlternateScreen,
            terminal::Clear(terminal::ClearType::All)
        )?;            
        terminal::disable_raw_mode()
    }

    fn draw_char(stdout: &mut Stdout, row: u32, column: u32, symbol: char, color: style::Color) -> io::Result<()> {
        execute!(
            stdout,
            cursor::MoveTo(column as u16, row as u16),            
            style::SetForegroundColor(color),
            style::Print(symbol)
        )
    }

    pub fn game_over_screen(&mut self, score: u16) {
        if let Err(e) = self.shutdown() {
            eprintln!("error while cleaning screen to show game over screen: {e}");
        };
        println!("game over, your final score was: {score}");        
    }
}