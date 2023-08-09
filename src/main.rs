use winit::{event::{Event, DeviceEvent, WindowEvent, VirtualKeyCode}, event_loop::EventLoop};
use rayon::prelude::*;
use image::{ImageBuffer, Rgba};

mod window;

pub const WIDTH: u32 = 1920;
pub const HEIGHT: u32 = 1080;

pub const XMIN: f64 = -2.5;
pub const XMAX: f64 = 1.0;
pub const YMIN: f64 = -1.0;
pub const YMAX: f64 = 1.0;

const MAX_ITER_PREVIEW: usize = 600;

fn calculate_pixel(mut x: f64, mut y:f64, x0: f64, y0: f64, max_iterations: usize) -> usize {
    let mut iterations = 0;
    let mut x2 = 0.;
    let mut y2 = 0.;
    while x2 + y2 <= 4. && iterations < max_iterations {
        y = 2. * x * y + y0;
        x = x2 - y2 + x0;
        x2 = x * x;
        y2 = y * y;
        iterations += 1;
    }
    iterations
}

fn get_scaled_point(px: usize, py: usize, zoom: f64, center: (f64, f64)) -> (f64, f64) {
    let scale_x = (px as f64 / WIDTH as f64) * (XMAX - XMIN) / zoom + center.0;
    let scale_y = (py as f64 / HEIGHT as f64) * (YMAX - YMIN) / zoom + center.1;
    (scale_x, scale_y)
}

fn draw_mandelbrot(frame: &mut [u8], zoom: f64, center: (f64, f64), max_iterations: usize) {
    let mut subpixels = frame.par_chunks_mut(4).collect::<Vec<_>>();
    subpixels.par_iter_mut().with_min_len(10).enumerate().for_each(|(idx, pixel)| {
        let px = idx % WIDTH as usize;
        let py = idx / WIDTH as usize;

        let (x0, y0) = get_scaled_point(px, py, zoom, center);
        let iterations = calculate_pixel(0., 0., x0, y0, max_iterations);
        let color = iter_to_color(iterations);
        pixel.copy_from_slice(&color);
    })
}

fn iter_to_color(iter: usize) -> [u8; 4] {
    // iter is hue
    hsv_to_rgb(iter as f64, 1.0, 0.7)
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> [u8; 4] {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    [((r + m) * 255.0) as u8, ((g + m) * 255.0) as u8, ((b + m) * 255.0) as u8, 255]
}

fn take_snapshot(center: (f64, f64), zoom: f64) {
    const SNAPSHOT_WIDTH: u32 = 3840;  // For example, 4K resolution width
    const SNAPSHOT_HEIGHT: u32 = 2160; // For example, 4K resolution height
    const SNAPSHOT_MAX_ITER: usize = 1000; // Increased iteration count for more detail

    let mut buffer: Vec<u8> = vec![0; (SNAPSHOT_WIDTH * SNAPSHOT_HEIGHT * 4) as usize];

    draw_mandelbrot(&mut buffer, zoom, center, SNAPSHOT_MAX_ITER);
    
    save_to_image(&buffer, SNAPSHOT_WIDTH, SNAPSHOT_HEIGHT);
}


fn save_to_image(buffer: &[u8], width: u32, height: u32) {
    let img = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, buffer).unwrap();
    img.save("snapshot.png").unwrap();
}

fn main() {
    let event_loop = EventLoop::new();
    let mut window = window::GameWindow::new("Mandelbrot", &event_loop).unwrap();

    // let mut center = ((XMAX + XMIN) / 2.0, (YMAX + YMIN) / 2.0);
    let mut center = (0. , 0.);
    let mut zoom = 1.;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                let frame = window.pixels.frame_mut();
                draw_mandelbrot(frame, zoom, center, MAX_ITER_PREVIEW);
                window.pixels.render().unwrap();
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = winit::event_loop::ControlFlow::Exit,

            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                window.resize(size.into());
            }

            Event::WindowEvent { 
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => {
                        zoom *= 1. + y as f64 * 0.1;

                        window.window.request_redraw();
                    }
                    _ => {}
                }
            }

            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                if !center.0.is_normal() || !center.1.is_normal() {
                    center = ((XMAX + XMIN) / 2.0, (YMAX + YMIN) / 2.0);
                }
                center.0 += delta.0 as f64 / WIDTH as f64 * (XMAX - XMIN) / zoom;
                center.1 += delta.1 as f64 / HEIGHT as f64 * (YMAX - YMIN) / zoom; // Note the subtraction here

                window.window.request_redraw();
            }

            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    input,
                    ..
                },
                ..
            } => {
                if let Some(keycode) = input.virtual_keycode {
                    match keycode {
                        VirtualKeyCode::Space => {
                            take_snapshot(center, zoom);
                        },

                        VirtualKeyCode::Escape => {
                            *control_flow = winit::event_loop::ControlFlow::Exit;
                        },

                        _ => {}
                    }
                }
            },
            
            _ => {}
        }
    })
}
