struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
};

struct Transform {
    position: vec4<f32>,
    rotation: vec4<f32>,  // Assuming Euler angles in radians
    scale: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> transform: Transform;

@vertex
fn vert_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Create rotation matrix from Euler angles
    let cosX = cos(transform.rotation.x);
    let sinX = sin(transform.rotation.x);
    let cosY = cos(transform.rotation.y);
    let sinY = sin(transform.rotation.y);
    let cosZ = cos(transform.rotation.z);
    let sinZ = sin(transform.rotation.z);

    let rotationX = mat4x4<f32>(
        vec4<f32>(1.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, cosX, -sinX, 0.0),
        vec4<f32>(0.0, sinX, cosX, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );

    let rotationY = mat4x4<f32>(
        vec4<f32>(cosY, 0.0, sinY, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(-sinY, 0.0, cosY, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );

    let rotationZ = mat4x4<f32>(
        vec4<f32>(cosZ, -sinZ, 0.0, 0.0),
        vec4<f32>(sinZ, cosZ, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );

    let rotation = rotationZ * rotationY * rotationX;

    // Apply scale
    let scaleMatrix = mat4x4<f32>(
        vec4<f32>(transform.scale.x, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, transform.scale.y, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, transform.scale.z, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );

    // Combine rotation and scale
    let modelMatrix = scaleMatrix * rotation;

    // Translation
    let translation = mat4x4<f32>(
        vec4<f32>(1.0, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(transform.position.x, transform.position.y, transform.position.z, 1.0)
    );

    // Full model transformation matrix
    let fullModelMatrix = translation * modelMatrix;


        // Define the perspective projection matrix for window size 1600x1200
        let fov: f32 = 1.0 / tan(0.7854 / 2.0);  // 45 degrees field of view
        let aspect_ratio: f32 = 1600.0 / 1200.0;  // Aspect ratio calculated from window dimensions
        let zNear: f32 = 0.1;
        let zFar: f32 = 100.0;
        let zRange: f32 = zNear - zFar;
        let perspective = mat4x4<f32>(
            vec4<f32>(fov / aspect_ratio, 0.0, 0.0, 0.0),
            vec4<f32>(0.0, fov, 0.0, 0.0),
            vec4<f32>(0.0, 0.0, (zFar + zNear) / zRange, -1.0),
            vec4<f32>(0.0, 0.0, (2.0 * zFar * zNear) / zRange, 0.0)
        );

    // Apply transformations
    out.clip_position = perspective * fullModelMatrix * vec4<f32>(model.position, 1.0);
    out.normal = normalize((modelMatrix * vec4<f32>(model.normal, 0.0)).xyz);

    return out;
}

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple directional light
    let light_dir = normalize(vec3<f32>(0.5, 0.5, 1.0));
    let light_color = vec3<f32>(1.0, 1.0, 1.0);
    let ambient_color = vec3<f32>(0.1, 0.1, 0.1);

    // Lambertian shading
    let normal = normalize(in.normal);
    let light_intensity = max(dot(normal, light_dir), 0.0);

    let color = vec3<f32>(
        1.0, 1.0, 1.0
    );


    let final_color = vec4<f32>((ambient_color + light_intensity * light_color) * color, 1.0);

    return final_color;
}
