struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) fragCoord: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.fragCoord = model.position.xy;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let max_iterations = 1000;
    let scale = vec2<f32>(3.0, 3.0); // Expand to Mandelbrot's range
    let offset = vec2<f32>(-2.0, -1.5); // Centering the Mandelbrot set
    let c = in.fragCoord * scale + offset;
    var z = vec2<f32>(0.0, 0.0);
    var i: i32 = 0;

    while (i < max_iterations && dot(z, z) < 4.0) {
        z = vec2<f32>(z.x*z.x - z.y*z.y, 2.0*z.x*z.y) + c;
        i = i + 1;
    }

    if (i == max_iterations) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0); // Black for points inside the Mandelbrot set
    } else {
        let color = f32(i) / f32(max_iterations); // Convert iteration number into an RGB color
        // return vec4<f32>(color, color, color, 1.0);
        return vec4<f32>(1.0, 0.0, 0.0, 1.0); // red color
    }
}
