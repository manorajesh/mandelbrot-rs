struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) color: vec3<f32>,
};

struct FragmentInput {
    @builtin(position) clip_position: vec4<f32>,
};

struct Uniforms {
    width: f32,
    height: f32,
    zoom: f32,
    center_x: f32,
    center_y: f32,
}

@group(0) @binding(0)
var<uniform> u: Uniforms;

@vertex
fn vs_main(
    model: VertexInput,
) -> FragmentInput {
    var out: FragmentInput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: FragmentInput) -> @location(0) vec4<f32> {
    let max_iterations = 10000;  // Or some appropriate value
    let XMIN = -2.5;  // Adjust to your actual Mandelbrot coordinates
    let XMAX = 1.0;  // Adjust to your actual Mandelbrot coordinates
    let YMIN = -1.0;  // Adjust to your actual Mandelbrot coordinates
    let YMAX = 1.0;  // Adjust to your actual Mandelbrot coordinates

    // Scale to fit the Mandelbrot coordinates
    let x0 = (in.clip_position[0] / u.width) * (XMAX - XMIN) / u.zoom + u.center_x;
    let y0 = (in.clip_position[1] / u.height) * (YMAX - YMIN) / u.zoom + u.center_y;

    var x = 0.0;
    var y = 0.0;
    var iterations = 0;
    var x2 = 0.0;
    var y2 = 0.0;

    while x2 + y2 <= 4.0 && iterations < max_iterations {
        y = 2.0 * x * y + y0;
        x = x2 - y2 + x0;
        x2 = x * x;
        y2 = y * y;
        iterations = iterations + 1;
    }

    // Convert iterations to a color (simple linear grayscale for now)
    let color_value = f32(iterations) / f32(max_iterations);
    let color = vec4<f32>(color_value, color_value, color_value, 1.0);

    // Implement your HSV to RGB conversion here if you need that.

    return color;
}