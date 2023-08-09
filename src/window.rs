use pixels::{Error, Pixels, SurfaceTexture, PixelsBuilder};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{CursorGrabMode, Window, WindowBuilder},
};

use crate::{HEIGHT, WIDTH};

pub struct GameWindow {
    pub window: Window,
    pub pixels: Pixels,
}

impl GameWindow {
    pub fn new(title: &str, event_loop: &EventLoop<()>) -> Result<Self, Error> {
        let window = {
            let size: LogicalSize<f64> = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
            WindowBuilder::new()
                .with_title(title)
                .with_min_inner_size(LogicalSize::new(100, 100))
                .with_inner_size(size)
                .build(event_loop)
                .unwrap()
        };

        // let mut scale_factor = window.scale_factor();
    
        window
            .set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
            .unwrap();
        window.set_cursor_visible(false);

        let pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            PixelsBuilder::new(WIDTH as u32, HEIGHT as u32, surface_texture)
                .enable_vsync(true)
                .build()?
        };
    
        // let surface_texture = SurfaceTexture::new(WIDTH/2, HEIGHT/2, &window);
        // let pixels = PixelsBuilder::new(size.width, size.height, surface_texture)
        //     .enable_vsync(true)
        //     .build()?;
    
        Ok(Self {
            window,
            pixels,
        })
    }    

    pub fn resize(&mut self, new_size: (u32, u32)) {
        let size = LogicalSize::new(new_size.0, new_size.1);
        // self.pixels.resize_buffer(size.width, size.height).unwrap();
        self.pixels.resize_surface(size.width, size.height).unwrap();
    }    
}