struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,  
};

@vertex
fn vertex_main(@location(0) vertex_pos: vec3<f32>) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(vertex_pos, 1.0);

    return out;
}

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Red
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}