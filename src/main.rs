#[macro_use]
extern crate gfx;

pub mod system;

use system::game_system::GameBuilder;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut game = GameBuilder::new()
        .with_title("Tick Tack Toe")
        .with_size(800, 600)
        .with_bpp(24)
        .in_fullscreen(false)
        .build();

    game.run();
}
