use core::types::WindowSize;
use system::render_system::RenderSystem;

pub struct Game {
    is_running: bool,

    render_system: RenderSystem,
}

impl Game {
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn update(&self) {
        self.handle_events();
        self.render()
    }

    fn handle_events(&self) {
        //self.render_system.handle_event();
    }

    fn render(&self) {
        self.render_system.render()
    }
}

pub struct GameBuilder {
    window_size: WindowSize,
    fullscreen: bool,
    bpp: u8
}

impl GameBuilder {
    pub fn new() -> GameBuilder {
        GameBuilder {
            window_size: WindowSize { w: 800, h: 600 },
            fullscreen: false,
            bpp: 24
        }
    }

    pub fn with_size(&mut self, w: u32, h: u32) -> &mut GameBuilder {
        self.window_size.w = w;
        self.window_size.h = h;

        self
    }

    pub fn with_bpp(&mut self, bpp: u8) -> &mut GameBuilder {
        self.bpp = bpp;

        self
    }

    pub fn in_fullscreen(&mut self, fullscreen: bool) -> &mut GameBuilder {
        self.fullscreen = fullscreen;

        self
    }

    pub fn finalise(&self) -> Game {


        let mut render_system = RenderSystem::new(self.window_size, );

        Game {
            is_running: false,
            render_system: render_system
        }
    }
}