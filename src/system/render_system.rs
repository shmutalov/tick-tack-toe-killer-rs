use system::gfx as gfx;
use system::gfx_glutin as gfx_glutin;
use system::gfx_gl as gfx_gl;
use system::glutin as glutin;
use system::glutin::{Event};
use system::gfx::handle::ShaderResourceView;
use system::gfx::traits::FactoryExt;
use system::gfx::Device;

use system::core::types::{WindowSize, Size2};
use system::core::pipeline::{pipe, ColorFormat, DepthFormat};
use system::cgmath::{Matrix4, SquareMatrix};

pub struct RenderSystem {
    clear_color: [f32; 4],
    window_size: WindowSize,

    device: gfx_gl::Device,
    encoder: gfx::Encoder<gfx_gl::Resources, gfx_gl::CommandBuffer>,
    data: pipe::Data<gfx_gl::Resources>,
}

impl RenderSystem {
    pub fn new(size: WindowSize,
               mut device: gfx_gl::Device,
               mut factory: gfx_gl::Factory,
               main_color: gfx::handle::RenderTargetView<gfx_gl::Resources, ColorFormat>,
               main_depth: gfx::handle::DepthStencilView<gfx_gl::Resources, DepthFormat>,
               encoder: gfx::Encoder<gfx_gl::Resources, gfx_gl::CommandBuffer>)
               -> RenderSystem {
        let vb = factory.create_vertex_buffer(&[]);
        let sampler = factory.create_sampler_linear();
        let fake_texture = RenderSystem::load_texture_raw(&mut factory, Size2 { w: 2, h: 2 }, &[0; 4]);

        let data = pipe::Data {
            basic_color: [1.0, 1.0, 1.0, 1.0],
            vbuf: vb,
            texture: (fake_texture, sampler),
            out: main_color,
            out_depth: main_depth,
            mvp: Matrix4::identity().into(),
        };

        RenderSystem {
            clear_color: [0.0, 0.0, 0.0, 1.0],
            window_size: size,
            device: device,
            encoder: encoder,
            data: data,
        }
    }

    pub fn render(&mut self, window: &glutin::Window) {
        self.encoder.clear(&self.data.out, self.clear_color);
        self.encoder.clear_depth(&self.data.out_depth, 1.0);

        self.encoder.flush(&mut self.device);
        window.swap_buffers()
            .expect("Cannot swap buffers");
        self.device.cleanup();
    }

    pub fn resize(&mut self, window: &glutin::Window, w: u32, h: u32) {
        if w == 0 || h == 0 {
            return;
        }

        self.window_size.w = w;
        self.window_size.h = h;

        gfx_glutin::update_views(
            window,
            &mut self.data.out,
            &mut self.data.out_depth
        );
    }

    pub fn deinitialize(&mut self) {}

    fn load_texture_raw<R, F>(factory: &mut F, size: Size2, data: &[u8]) -> ShaderResourceView<R, [f32; 4]>
        where R: gfx::Resources, F: gfx::Factory<R>
    {
        let kind = gfx::tex::Kind::D2(size.w as gfx::tex::Size, size.h as gfx::tex::Size, gfx::tex::AaMode::Single);
        let (_, view) = factory.create_texture_const_u8::<ColorFormat>(kind, &[data]).unwrap();

        view
    }
}