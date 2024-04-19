struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
};

struct InstanceInput{
    @builtin(instance_index) instance_index: u32,
    @location(3) model_matrix_0: vec4<f32>,
    @location(4) model_matrix_1: vec4<f32>,
    @location(5) model_matrix_2: vec4<f32>,
    @location(6) model_matrix_3: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) instance_color: vec3<f32>,
};

struct Camera{
    proj_view: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: Camera;


@vertex
fn vert_main(model: VertexInput, inst: InstanceInput) -> VertexOutput {
    var out: VertexOutput;

    // Build a transformation matrix from the instance input
    let modelMatrix = mat4x4<f32>(
        vec4<f32>(inst.model_matrix_0.x, inst.model_matrix_0.y, inst.model_matrix_0.z, inst.model_matrix_0.w),
        vec4<f32>(inst.model_matrix_1.x, inst.model_matrix_1.y, inst.model_matrix_1.z, inst.model_matrix_1.w),
        vec4<f32>(inst.model_matrix_2.x, inst.model_matrix_2.y, inst.model_matrix_2.z, inst.model_matrix_2.w),
        vec4<f32>(inst.model_matrix_3.x, inst.model_matrix_3.y, inst.model_matrix_3.z, inst.model_matrix_3.w)
    );


    // Apply transformations
    out.clip_position =  camera.proj_view * modelMatrix * vec4<f32>(model.position, 1.0);
    out.normal = normalize((modelMatrix * vec4<f32>(model.normal, 0.0)).xyz);
    out.tex_coords = model.tex_coords;

    // use instance_index (built-in) to determine the index
    var color = vec3<f32>(sin(f32(inst.instance_index)), cos(f32(inst.instance_index)), 0.5);
    out.instance_color = color;

    return out;
}


fn aces_tonemapping(color: vec3<f32>) -> vec3<f32> {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    return (color * (color * a + b)) / (color * (color * c + d) + e);
}

// Filters
fn linear_to_srgb(color: vec3<f32>) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / 2.2));
}

fn invert(color: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(1.0) - color;
}

fn grayscale(color: vec3<f32>) -> vec3<f32> {
    let average = (color.r + color.g + color.b) / 3.0;
    return vec3<f32>(average);
}

fn sepia(color: vec3<f32>) -> vec3<f32> {
    let r = color.r * 0.393 + color.g * 0.769 + color.b * 0.189;
    let g = color.r * 0.349 + color.g * 0.686 + color.b * 0.168;
    let b = color.r * 0.272 + color.g * 0.534 + color.b * 0.131;
    return vec3<f32>(r, g, b);
}

fn brightness(color: vec3<f32>, value: f32) -> vec3<f32> {
    return color + vec3<f32>(value);
}

fn contrast(color: vec3<f32>, value: f32) -> vec3<f32> {
    return (color - vec3<f32>(0.5)) * value + vec3<f32>(0.5);
}

fn saturation(color: vec3<f32>, value: f32) -> vec3<f32> {
    let average = (color.r + color.g + color.b) / 3.0;
    return vec3<f32>(average) + (color - vec3<f32>(average)) * value;
}

// Texture and sampler
@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple directional light
    let light_dir = normalize(vec3<f32>(0.0, 0.5, 1.0));
    let light_color = vec3<f32>(1.0, 1.0, 1.0);
    let ambient_color = vec3<f32>(0.1, 0.1, 0.1);

    // Lambertian shading
    let normal = normalize(in.normal);
    let light_intensity = max(dot(normal, light_dir), 0.0);

    let color = in.instance_color;


    let adjusted_color = vec3<f32>((ambient_color + light_intensity * light_color) * color);

    // Apply filters
    var final_color = adjusted_color;
    // Apply texture
    let tex_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    final_color *= tex_color.rgb;

    return vec4<f32>(aces_tonemapping(final_color), 1.0);
}
