use std::mem;

/// ライフゲームのアルゴリズム
/// 現在のstateを元に次の瞬間のstate(next_state)を計算しstateとnext_stateを入れ替える
/// # Arguments
/// * `state` - 現在の状態
/// * `next_state` - 次の瞬間の状態
/// * `height` - セルの縦の数
/// * `width` - セルの横の数
pub fn game_of_life(state: &mut Vec<Vec<u8>>, next_state: &mut Vec<Vec<u8>>, height: usize, width: usize) {
    for i in 0..height {
        for j in 0..width {
            let nw = state[(i + height - 1) % height][(j + width - 1) % width];
            let n = state[(i + height - 1) % height][j];
            let ne = state[(i + height - 1) % height][(j + 1) % width];
            let w = state[i][(j + width - 1) % width];
            let c = state[i][j];
            let e = state[i][(j + 1) % width];
            let sw = state[(i + 1) % height][(j + width - 1) % width];
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
    mem::swap(state, next_state);
}

/// ライフゲームのアルゴリズム
/// 現在のstateを元に次の瞬間のstate(next_state)を計算し返り値として返す
/// * next_stateを計算するのに、複数threadを用いて各行の値を計算し(fork)、計算結果が集まるのを待つ(join)
/// * stateを変更することはない
///   * stateをimmutableにして複数threadで共有できる
///     * [std::sync::Arc(Atomic Reference Counted)](https://doc.rust-lang.org/std/sync/struct.Arc.html)
///   * immutableパターン
///
/// state                                  next_state
///   0 1 2                                  0 1 2
/// 0 - - -  -┐   calculated by thread1--> 0 - - -
/// 1 - - -  ---> calculated by thread2--> 1 - - -
/// 2 - - -  -┘   calculated by thread3--> 2 - - -
/// # Arguments
/// * `state` - 現在の状態(Arc(参照のようなもの)に包まれている)
/// * `height` - セルの縦の数
/// * `width` - セルの横の数
use std::sync::Arc;
pub fn game_of_life_in_parallel(state: Arc<Vec<Vec<u8>>>, height: usize, width: usize) -> Vec<Vec<u8>> {
    use std::thread;

    let mut thread_handles = vec![];
    for i in 0..height {
        // cloneしてもdata自体がcloneされるわけではなく、参照のようなものがcloneされる
        // 参照のようなものがいくつ使われているかはcountされている(実行時コストがかかる)。countが0になったら中身をdropする
        let cloned_state = state.clone();
        // 高さの数だけthreadを生成する
        thread_handles.push(thread::spawn(move || {
            let mut row: Vec<u8> = Vec::with_capacity(width);
            for j in 0..width {
                let nw = cloned_state[(i + height - 1) % height][(j + width - 1) % width];
                let n = cloned_state[(i + height - 1) % height][j];
                let ne = cloned_state[(i + height - 1) % height][(j + 1) % width];
                let w = cloned_state[i][(j + width - 1) % width];
                let c = cloned_state[i][j];
                let e = cloned_state[i][(j + 1) % width];
                let sw = cloned_state[(i + 1) % height][(j + width - 1) % width];
                let s = cloned_state[(i + 1) % height][j];
                let se = cloned_state[(i + 1) % height][(j + 1) % width];
                let neighbor_cell_sum = nw + n + ne + w + e + sw + s + se;
                if c == 0 && neighbor_cell_sum == 3 {
                    row.push(1);
                } else if c == 1 && (neighbor_cell_sum == 2 || neighbor_cell_sum == 3) {
                    row.push(1);
                } else {
                    row.push(0);
                }
            }
            // threadからの出力(i(高さ)は使っていないので実際には不要だが、待受側で順番が変わっていないか見るのに使える)
            (i, row)
        }));
    }
    thread_handles
        .into_iter()
        .map(|h| h.join().unwrap().1) // threadの終了を待ち受ける。
        .collect::<Vec<Vec<_>>>()

    // ↓みたいに書いても良い
    // let next_state: Vec<Vec<u8>> = Vec::with_capacity(height);
    // for handle in thread_handles {
    //     next_state.push(handle.join().unwrap().1);
    // }
    // next_state
}

/// ライフゲームのアルゴリズム
/// 現在のstateを元に次の瞬間のstate(next_state)を計算し返り値として返す
/// [Rayon](https://docs.rs/rayon/1.0.3/rayon/)というfolk-join parallelismに適したlibraryを使う
/// * stateの参照を複数threadで共有できる(Arcに包む必要がない)
/// # Arguments
/// * `state` - 現在の状態への参照
/// * `height` - セルの縦の数
/// * `width` - セルの横の数
pub fn game_of_life_by_rayon(state: &Vec<Vec<u8>>, height: usize, width: usize) -> Vec<Vec<u8>> {
    use rayon::prelude::*;
    (0..height)
        .into_par_iter() // 通常のinto_iter()をinto_par_iter()にするだけ
        .map(|i| {
            let mut row: Vec<u8> = Vec::with_capacity(width);
            for j in 0..width {
                let nw = state[(i + height - 1) % height][(j + width - 1) % width];
                let n = state[(i + height - 1) % height][j];
                let ne = state[(i + height - 1) % height][(j + 1) % width];
                let w = state[i][(j + width - 1) % width];
                let c = state[i][j];
                let e = state[i][(j + 1) % width];
                let sw = state[(i + 1) % height][(j + width - 1) % width];
                let s = state[(i + 1) % height][j];
                let se = state[(i + 1) % height][(j + 1) % width];
                let neighbor_cell_sum = nw + n + ne + w + e + sw + s + se;
                if c == 0 && neighbor_cell_sum == 3 {
                    row.push(1);
                } else if c == 1 && (neighbor_cell_sum == 2 || neighbor_cell_sum == 3) {
                    row.push(1);
                } else {
                    row.push(0);
                }
            }
            row
        }).collect::<Vec<_>>()
}
