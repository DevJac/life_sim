struct LineVertex {
    @location(0) position: vec2f,
}

struct LineInstance {
    @location(0) a: vec2f,
    @location(1) b: vec2f,
    @location(2) color: vec3f,
}




// The size of the full low res canvas; e.g., 800 x 600.
// struct Camera {
//     @location(0) top_left: vec2f,
//     @location(1) width_height: vec2f,
// };
// 
// struct TextureVertex {
//     @location(0) position: vec3f,
//     @location(1) uv: vec2f,
//     @location(2) lower_right: vec3u,
// };
// 
// struct TextureFragment {
//     @builtin(position) position: vec4f,
//     @location(1) uv: vec2f,
//     @location(2) @interpolate(flat) lower_right: vec3u,
// };
// 
// @group(0) @binding(0) var<uniform> camera: Camera;
// @group(0) @binding(1) var textures_sampler: sampler;
// @group(0) @binding(2) var textures: texture_2d_array<f32>;
// 
// @vertex
// fn vertex_main(vertex: TextureVertex) -> TextureFragment {
//     // Adjust coordinates in our world space (e.g., somewhere in the 800 x 600 grid)
//     // to normalized device coordinates (NDC, e.g., somewhere in the -1 to 1 range).
//     let ndc = vec4f(
//         (vertex.position.x - camera.top_left.x) / f32(camera.width_height.x) * 2.0 - 1.0,
//         (vertex.position.y - camera.top_left.y) / f32(camera.width_height.y) * 2.0 - 1.0,
//         vertex.position.z,
//         1.0,
//     );
//     return TextureFragment(ndc, vertex.uv, vertex.lower_right);
// }
// 
// @fragment
// fn fragment_main(fragment: TextureFragment) -> @location(0) vec4f {
//     let full_dims: vec2u = textureDimensions(textures);
//     let adjusted_uv = vec2f(
//         fragment.uv.x * (f32(fragment.lower_right.x) / f32(full_dims.x)),
//         fragment.uv.y * (f32(fragment.lower_right.y) / f32(full_dims.y)),
//     );
//     return textureSample(textures, textures_sampler, adjusted_uv, fragment.lower_right.z);
// }
// 
// @fragment
// fn fragment_line(fragment: TextureFragment) -> @location(0) vec4f {
//     return vec4f(1.0, 1.0, 0.0, 1.0);
// }
