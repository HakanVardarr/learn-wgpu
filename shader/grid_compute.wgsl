@group(0) @binding(0) var<uniform> grid: vec2<f32>;

@group(0) @binding(1) var<storage> cell_state_in: array<u32>;
@group(0) @binding(2) var<storage, read_write> cell_state_out: array<u32>;

fn cell_index(cell: vec2<u32>) -> u32 {
    return (cell.y % u32(grid.y)) * u32(grid.x) +
         (cell.x % u32(grid.x));

}

fn cell_active(x: u32, y: u32) -> u32 {
    return cell_state_in[cell_index(vec2u(x, y))];
}

@compute
@workgroup_size(8, 8, 1)
fn compute_main(
    @builtin(global_invocation_id) cell: vec3<u32>,
) {
    let active_neighbors = cell_active(cell.x+1, cell.y+1) +
                        cell_active(cell.x+1, cell.y) +
                        cell_active(cell.x+1, cell.y-1) +
                        cell_active(cell.x, cell.y-1) +
                        cell_active(cell.x-1, cell.y-1) +
                        cell_active(cell.x-1, cell.y) +
                        cell_active(cell.x-1, cell.y+1) +
                        cell_active(cell.x, cell.y+1);


    let i = cell_index(cell.xy);

    switch active_neighbors {
        case 2u: {
            cell_state_out[i] = cell_state_in[i];
        }
        case 3u: {
            cell_state_out[i] = 1;
        }
        default: {
            cell_state_out[i] = 0;
        }
    }

}