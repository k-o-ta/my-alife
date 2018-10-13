use ndarray::Array1;
use std::mem;

/// セルラーオートマトンのルール
/// あるセルが次のフェーズで生きている「1」か、死んでいる「0」かは、そのセルと両脇のセルの生死によって決まる  
/// つまり生死には8通りのパターンがある。  
/// 000 -> (1), 001 -> (2), 010 -> (3), 011 -> (4), 100 -> (5), 101 -> (6), 110 -> (7), 111 -> (8)  
/// (1)から(8)がそれぞれ生きている「1」か死んでいるか「0」は任意に決めて良いので、2^8 = 256通りのルールを取りうる  
/// ルール1: 000 -> 0, 001 -> 0, 010 -> 0, 011 -> 0, 100 -> 0, 101 -> 0, 110 -> 0, 111 -> 1  
/// ルール2: 000 -> 0, 001 -> 0, 010 -> 0, 011 -> 0, 100 -> 0, 101 -> 0, 110 -> 1, 111 -> 0  
/// ルール3: 000 -> 0, 001 -> 0, 010 -> 0, 011 -> 0, 100 -> 0, 101 -> 0, 110 -> 1, 111 -> 1  
/// ...
/// 1行あたり16セルのルール1を採用した場合、  
/// フェーズ01: `0111011100000000`   
/// フェーズ02: `0101010100000000`   
/// フェーズ03: `0101010100000000`   
/// ...  
/// フェーズ16: `0101010100000000`  
/// で16 * 16の1面が完成する。ただしこの関数ではフェーズ1回分の計算だけを行い呼び出し側でループする
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
