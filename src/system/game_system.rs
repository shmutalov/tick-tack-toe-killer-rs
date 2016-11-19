use system::render_system::RenderSystem;
use system::gfx as gfx;
use system::glutin as glutin;
use system::gfx_glutin as gfx_glutin;

use system::glutin::Event;

use system::core::types::WindowSize;
use system::core::pipeline::{ColorFormat, DepthFormat};

pub struct Game {
    is_running: bool,
    window: glutin::Window,
    render_system: RenderSystem,
}

impl Game {
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn update(&mut self) {
        self.handle_events();
        self.render();
    }

    fn handle_events(&mut self) {
        let events = self.window.poll_events();

        for event in events {
            match event {
                Event::Closed => {
                    self.is_running = false;
                    self.render_system.deinitialize();
                },
                Event::Resized(w, h) => {
                    self.render_system.resize(&self.window, w, h);
                }
                _ => {},
            }
        }
    }

    fn render(&mut self) {
        self.render_system.render(&self.window);
    }

    pub fn run(&mut self) {
        self.is_running = true;

        while self.is_running {
            self.update();
            ::std::thread::sleep_ms(1);
        }
    }
}

pub struct GameBuilder {
    window_size: WindowSize,
    fullscreen: bool,
    bpp: u8,
    title: String
}

impl GameBuilder {
    pub fn new() -> GameBuilder {
        GameBuilder {
            window_size: WindowSize { w: 800, h: 600 },
            fullscreen: false,
            bpp: 24,
            title: "Game".to_string()
        }
    }

    pub fn with_title(&mut self, title: &str) -> &mut GameBuilder {
        self.title = title.to_string();

        self
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

    pub fn build(&self) -> Game {
        let builder = glutin::WindowBuilder::new()
            .with_title(self.title.clone())
            .with_dimensions(self.window_size.w, self.window_size.h)
            .with_pixel_format(self.bpp, 8)
            .with_vsync();

        let (window,
            mut device,
            mut factory,
            main_color,
            main_depth) = gfx_glutin::init::<ColorFormat, DepthFormat>(builder);

        let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        let mut render_system = RenderSystem::new(self.window_size, device, factory, main_color, main_depth, encoder);

        let game = Game {
            is_running: false,
            window: window,
            render_system: render_system,
        };

        game
    }
}