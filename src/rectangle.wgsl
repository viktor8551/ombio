@group(0) @binding(0) var<uniform> viewport: vec4<f32>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(
    @builtin(vertex_index) index: u32,
    @location(0) position: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) color: vec4<f32>,
) -> VertexOutput {
    let corners = array<vec2<f32>, 6>(
        vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(0.0, 1.0),
        vec2(0.0, 1.0), vec2(1.0, 0.0), vec2(1.0, 1.0),
    );
    let pixel = position + corners[index] * size;
    let clip = vec2(pixel.x / viewport.x * 2.0 - 1.0, 1.0 - pixel.y / viewport.y * 2.0);
    return VertexOutput(vec4(clip, 0.0, 1.0), color);
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
