pub mod Board;
pub mod Game;
pub mod Ship;
pub mod Difficulty;
pub mod AI;

fn main() {
    let mut game = Game::Game::new();

    game.start_game();
}
