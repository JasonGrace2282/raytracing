struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
  // gl_Position
  @builtin(position) clip_position: vec4<f32>,
  @location(0) tex_coords: vec2<f32>,
};

// uniforms for the fragment shader
// group(x) where x = index for set_bind_group
// binding(x) where x = binding in BindGroup

// texture diffuse
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;

// filterer
@group(0) @binding(1)
var d_diffuse: sampler;

@vertex
fn vs_main(
  model: VertexInput,
) -> VertexOutput {
  var out: VertexOutput;
  out.tex_coords = model.tex_coords;
  out.clip_position = vec4<f32>(model.position, 1.0);
  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return textureSample(t_diffuse, d_diffuse, in.tex_coords);
}
