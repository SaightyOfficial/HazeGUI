struct ScreenSize {
    width: f32,
    height: f32,
};

@group(0) @binding(0)
var<uniform> screen: ScreenSize;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;

    let norm_x = model.position.x / screen.width;
    let norm_y = model.position.y / screen.height;

    let clip_x = (norm_x * 2.0) - 1.0;
    let clip_y = 1.0 - (norm_y * 2.0);

    out.clip_position = vec4<f32>(clip_x, clip_y, model.position.z, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}