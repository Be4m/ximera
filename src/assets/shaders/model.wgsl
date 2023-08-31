struct CameraUniform {
    vp_matrix: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> model_matrix: mat4x4<f32>;

// @group(1) @binding(0) var<uniform> camera: CameraUniform;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,  
};

@vertex
fn vertex_main(@location(0) vertex_pos: vec3<f32>) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(vertex_pos, 1.0) * model_matrix;

    return out;
}

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // White
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}