@group(0) @binding(0) var<uniform> grid: vec2<f32>;
@group(0) @binding(1) var<storage> cell_state: array<u32>;

struct VertexInput {
    @location(0) pos: vec2<f32>,
    @builtin(instance_index) instance: u32,
};

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) cell: vec2<f32>,
};

@vertex
fn vs_main(
    vertex_input: VertexInput,
) -> VertexOutput {
    let i = f32(vertex_input.instance);
    let state = f32(cell_state[vertex_input.instance]);

    let cell = vec2f(i % grid.x, floor(i / grid.x));
    let cell_offset = cell / grid * 2;
    let grid_pos = (vertex_input.pos * state + 1) / grid - 1 + cell_offset;

    var output: VertexOutput;
    output.pos = vec4f(grid_pos, 0.0, 1.0);
    output.cell = cell;
    return output;

}

@fragment
fn fs_main(fragment_input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4f(0.16, 0.16, 0.16, 1.0);
}
