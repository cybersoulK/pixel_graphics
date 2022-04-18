use pixel_graphics;

use game::Game;

mod game;

fn main() {

    let game = Game {};

    pixel_graphics::init(game);
}

    