use ndarray::Array1;
use std::mem;

/// セルラーオートマトンのルール
pub fn cellular_automata(state: &mut (Array1<u32>, Array1<u32>), rule: u8, space_size: usize) {
    let current_state = &mut state.0;
    let next_state = &mut state.1;
    for i in 0..space_size {
        let l = current_state[(i + space_size - 1) % space_size];
        let c = current_state[(i + space_size) % space_size];
        let r = current_state[(i + space_size + 1) % space_size];
        let neighbor_cell_code = (2u32.pow(2)) * l + (2u32.pow(1)) * c + 2u32.pow(0) * r;
        if ((rule >> neighbor_cell_code) & 1) == 1 {
            next_state[i] = 1;
        } else {
            next_state[i] = 0;
        }
    }
    mem::swap(current_state, next_state);
}
