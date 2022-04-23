use game::Game;

mod game;

fn main() {

    let game = Game {};

    let settings = RenderingSettings { front_face: FrontFace::CounterClockWise };

    pixel_graphics::init(game, settings);
}