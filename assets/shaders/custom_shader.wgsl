@group(2) @binding(0)
var<uniform> time: f32;

@fragment
fn fragment(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let r = 0.5 + 0.5 * sin(time + uv.x * 10.0);
    let g = 0.5 + 0.5 * sin(time + uv.y * 10.0);
    let b = 0.5 + 0.5 * sin(time);
    return vec4<f32>(r, g, b, cos(time));
}
