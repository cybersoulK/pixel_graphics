use game::Game;

mod game;

fn main() {

    let game = Game::new();
    pixel_graphics::init(game);
}