use game::Game;

mod components;
mod systems;
mod world;
mod renderer;
mod game;

fn main() {    
    let mut game = Game::new(30, 20);
    if let Err(e) = game.run() {
        eprintln!("error: {e}");
    }
}
