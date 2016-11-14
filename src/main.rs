pub mod system;
pub mod core;

use system::game_system::GameBuilder;

fn main() {
    let mut game = GameBuilder::new()
        .with_size(800, 600)
        .with_bpp(24)
        .in_fullscreen(false)
        .finalise();

    while game.is_running() {
        game.update()
    }
}
