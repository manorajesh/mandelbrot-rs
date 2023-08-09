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
    max_iterations: i32,
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
    let XMIN = -2.5;
    let XMAX = 1.0;
    let YMIN = -1.0;
    let YMAX = 1.0;

    let x0 = (in.clip_position[0] / u.width) * (XMAX - XMIN) / u.zoom + u.center_x;
    let y0 = (in.clip_position[1] / u.height) * (YMAX - YMIN) / u.zoom + u.center_y;

    var x = 0.0;
    var y = 0.0;
    var iterations = 0;
    var x2 = 0.0;
    var y2 = 0.0;

    while x2 + y2 <= 4.0 && iterations < u.max_iterations {
        y = 2.0 * x * y + y0;
        x = x2 - y2 + x0;
        x2 = x * x;
        y2 = y * y;
        iterations = iterations + 1;
    }

    let normalized_iter = f32(iterations) / f32(u.max_iterations);
    let hue = normalized_iter * 0.5;

    let color = hsv_to_rgb(hue, 1.0, 0.7); // Using a fast hsv_to_rgb function will also help

    return vec4<f32>(color, 1.0);
}


fn hsv_to_rgb(h: f32, s: f32, v: f32) -> vec3<f32> {
    let c = v * s;
    let x = c * (1.0 - abs((h * 6.0) % 2.0 - 1.0));
    let m = v - c;
    
    var color: vec3<f32>;

    if (h < 1.0/6.0) {
        color = vec3<f32>(c, x, 0.0);
    } else if (h < 2.0/6.0) {
        color = vec3<f32>(x, c, 0.0);
    } else if (h < 3.0/6.0) {
        color = vec3<f32>(0.0, c, x);
    } else if (h < 4.0/6.0) {
        color = vec3<f32>(0.0, x, c);
    } else if (h < 5.0/6.0) {
        color = vec3<f32>(x, 0.0, c);
    } else {
        color = vec3<f32>(c, 0.0, x);
    }

    return color + m;
}
