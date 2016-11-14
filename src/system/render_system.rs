extern crate gfx;
extern crate gfx_device_gl as gfx_gl;
extern crate glutin;
extern crate gfx_window_glutin as gfx_glutin;
extern crate time;

use core::types::WindowSize;
use self::glutin::{Window, Event};
use self::gfx_gl::{Factory, Device, Resources, CommandBuffer};
//use gfx_glutin;

pub struct RenderSystem {
    clear_color: [f32; 4],
    window_size: WindowSize,

    window: glutin::Window,
    device: gfx_gl::Device,
    factory: gfx_gl::Factory,
    encoder: gfx::Encoder<gfx_gl::Resources, gfx_gl::CommandBuffer>,
}

impl RenderSystem {
    pub fn new(size: WindowSize,
               window: glutin::Window,
               device: gfx_gl::Device,
               factory: gfx_gl::Factory,
               encoder: gfx::Encoder<gfx_gl::Resources, gfx_gl::CommandBuffer>)
               -> RenderSystem {
        RenderSystem {
            clear_color: [0.0, 0.0, 0.0, 1.0],
            window_size: size,
            window: window,
            device: device,
            factory: factory,
            encoder: encoder
        }
    }

    pub fn render(&mut self) {
        self.encoder.clear(&self.data.out, self.clear_color);
        self.encoder.clear_depth(&self.data.out_depth, 1.0);

        self.encoder.flush(&mut self.device);
        self.window.swap_buffers()
            .expect("Cannot swap buffers");
        self.device.cleanup();
    }

    pub fn handle_event(&mut self, event: &glutin::Event) {
        match *event {
            Event::Closed => {
                self.deinitialize();
            },
            Event::Resized(w, h) => {
                self.resize(w, h);
            }
            _ => {},
        }
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        if w == 0 || h == 0 {
            return;
        }

        self.window_size.w = w;
        self.window_size.h = h;

        gfx_glutin::update_views(
            &self.window,
            &mut self.data.out,
            &mut self.data.out_depth
        );
    }

    pub fn deinitialize(&mut self) {}
}