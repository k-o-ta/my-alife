pub fn game_of_life(state: &mut Vec<Vec<i32>>, next_state: &mut Vec<Vec<i32>>, height: usize, width: usize) {
    for i in 0..height {
        for j in 0..width {
            let nw = state[i - 1][j - 1];
            let n = state[i - 1][j];
            let ne = state[i - 1][(j + 1) % width];
            let w = state[i][j - 1];
            let c = state[i][j];
            let e = state[i][(j + 1) % width];
            let sw = state[(i + 1) % height][j - 1];
            let s = state[(i + 1) % height][j];
            let se = state[(i + 1) % height][(j + 1) % width];
            let neighbor_cell_sum = nw + n + ne + w + e + sw + s + se;
            if c == 0 && neighbor_cell_sum == 3 {
                next_state[i][j] = 1;
            } else if c == 1 && (neighbor_cell_sum == 2 || neighbor_cell_sum == 3) {
                next_state[i][j] = 1;
            } else {
                next_state[i][j] = 0;
            }
        }
    }
}
