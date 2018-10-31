use std::mem;

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

use std::sync::Arc;
pub fn game_of_life_in_parallel(state: Arc<Vec<Vec<u8>>>, height: usize, width: usize) -> Vec<Vec<u8>> {
    use std::sync::mpsc;
    use std::thread;
    // let (tx, rx) = mpsc::channel();
    // let arc_state = Arc::new(state);

    let mut thread_handles = vec![];
    for i in 0..height {
        let cloned_state = state.clone();
        thread_handles.push(thread::spawn(move || {
            let mut row: Vec<u8> = Vec::with_capacity(width);
            for j in 0..width {
                // let tx_clone = mpsc::Sender::clone(&tx);
                // let tx_clone = mpsc::Sender::clone(&tx);
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
                //     tx_clone.send(((i, j), 1)).unwrap();
                } else if c == 1 && (neighbor_cell_sum == 2 || neighbor_cell_sum == 3) {
                    row.push(1);
                //     tx_clone.send(((i, j), 1)).unwrap();
                } else {
                    row.push(0);
                    // tx_clone.send(((i, j), 0)).unwrap();
                    // tx_clone.send(0).unwrap();
                }
            }
            (i, row)
        }));
    }
    thread_handles
        .into_iter()
        .map(|h| h.join().unwrap().1)
        .collect::<Vec<Vec<_>>>()
    // for handles in thread_handles {
    //     println!("{:?}", handles.join());
    // }
    // vec![vec![1]]
    // for received in rx {}
    // mem::swap(state, next_state);
}

use rayon::prelude::*;
pub fn game_of_life_by_rayon(state: &Vec<Vec<u8>>, height: usize, width: usize) -> Vec<Vec<u8>> {
    use std::sync::mpsc;
    use std::thread;
    // let (tx, rx) = mpsc::channel();
    // let arc_state = Arc::new(state);

    (0..height)
        .into_iter()
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
    // vec![vec![1]]
    // for received in rx {}
    // mem::swap(state, next_state);
}

// fn life_in_row(state: &Vec<Vec<u8>>) -> Vec<u8> {}
