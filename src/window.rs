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
        let size = LogicalSize::new(WIDTH, HEIGHT);
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(size)
            .build(event_loop)
            .unwrap();
    
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
        let pixels = PixelsBuilder::new(WIDTH, HEIGHT, surface_texture)
            .enable_vsync(true)
            .build()?;
    
        Ok(Self {
            window,
            pixels,
        })
    }    

    pub fn resize(&mut self, new_size: (u32, u32)) {
        self.pixels.resize_surface(new_size.0, new_size.1).unwrap();
    }    
}
