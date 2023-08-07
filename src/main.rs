use winit::{event::{self, Event}, event_loop::EventLoop};

mod window;

pub const HEIGHT: u32 = 600;
pub const WIDTH: u32 = 800;

fn calculate_pixel(mut x: f64, mut y:f64, x0: f64, y0: f64, max_iterations: usize) -> usize {
    let mut iterations = 0;
    while x*x + y*y <= 4. && iterations < max_iterations {
        let xtemp = x*x - y*y + x0;
        y = 2.*x*y + y0;
        x = xtemp;
        iterations += 1;
    }
    iterations
}

fn get_scaled_point(px: usize, py: usize) -> (f64, f64) {
    const XMIN: f64 = -2.5;
    const XMAX: f64 = 1.0;
    const YMIN: f64 = -1.0;
    const YMAX: f64 = 1.0;
    
    let x0 = (px as f64 / WIDTH as f64) * (XMAX - XMIN) + XMIN;
    let y0 = (py as f64 / HEIGHT as f64) * (YMAX - YMIN) + YMIN;
    (x0, y0)
}

fn draw_mandelbrot(frame: &mut [u8]) {
    for (idx, pixel) in frame.chunks_mut(4).into_iter().enumerate() {
        let px = idx % WIDTH as usize;
        let py = idx / WIDTH as usize;

        let (x0, y0) = get_scaled_point(px, py);
        let iterations = calculate_pixel(0., 0., x0, y0, 255);
        pixel[0] = iterations as u8;
        pixel[1] = iterations as u8;
        pixel[2] = iterations as u8;
        pixel[3] = 255;
        // println!("{} {} {} {}", pixel[0], pixel[1], pixel[2], pixel[3]);
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let mut window = window::GameWindow::new("Mandelbrot", &event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                let frame = window.pixels.frame_mut();
                draw_mandelbrot(frame);
                window.pixels.render().unwrap();
                // println!("Redraw requested");
            }

            Event::WindowEvent {
                event: event::WindowEvent::CloseRequested,
                ..
            } => *control_flow = winit::event_loop::ControlFlow::Exit,

            Event::WindowEvent {
                event: event::WindowEvent::Resized(size),
                ..
            } => {
                window.resize(size.into());
            }

            _ => {}
        }

        window.window.request_redraw();
    })
}
