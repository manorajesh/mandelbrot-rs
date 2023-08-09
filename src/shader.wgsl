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
    let max_iterations = 1000;  // Or some appropriate value
    let zoom: f32 = 1.0;  // Adjust as needed
    // let center: vec2<f32> = vec2<f32>(0.0, 0.0);  // Center of the Mandelbrot view
    let XMIN = -2.5;  // Adjust to your actual Mandelbrot coordinates
    let XMAX = 1.0;  // Adjust to your actual Mandelbrot coordinates
    let YMIN = -1.0;  // Adjust to your actual Mandelbrot coordinates
    let YMAX = 1.0;  // Adjust to your actual Mandelbrot coordinates

    // Scale to fit the Mandelbrot coordinates
    let x0 = (in.clip_position[0] / 1000.0) * (XMAX - XMIN) / zoom; //+ center[0];
    let y0 = (in.clip_position[1] / 1000.0) * (YMAX - YMIN) / zoom; //+ center[1];

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