struct Line {
    a: vec2f,
    b: vec2f,
    color: vec4f,
}

struct VertexOut {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
}

@group(0) @binding(0) var<uniform> world_space_size: vec2f;
@group(0) @binding(1) var<storage> lines: array<Line>;

@vertex
fn vertex_main(
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) instance_index: u32,
) -> VertexOut {
    let line = lines[instance_index];
    var world_space_vertex: vec2f;
    if vertex_index == 0u {
	world_space_vertex = line.a;
    } else {
	world_space_vertex = line.b;
    }
    let normalized_vertex: vec2f = world_space_vertex / world_space_size;
    return VertexOut(vec4f(normalized_vertex, 0.0, 1.0), line.color);
}

@fragment
fn fragment_main(@location(0) color: vec4f) -> @location(0) vec4f {
    return color;
}
