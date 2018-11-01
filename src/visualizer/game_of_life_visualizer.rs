use algorithm::game_of_life::game_of_life_by_rayon;
use failure;
use ndarray::prelude::*;
use ndarray::{arr2, Array1, Array2, FixedInitializer};
use rand::{thread_rng, Rng};
use std::mem;
use std::sync::Arc;
use visualizer::matrix_visualizer::MatrixVisualizer;
use visualizer::WindowStatus;
const WIDTH: usize = 50;
const HEIGHT: usize = WIDTH;

// pub type Matrix = [[u8; WIDTH]; HEIGHT];
pub type Matrix = Vec<Vec<u8>>;
/// 1次元配列を用いてvisualizeする構造体
/// 内部的に1次元配列を2次元配列(Matrix)に変換する
pub struct GameOfLifeVisualizer {
    matrix_visualizer: MatrixVisualizer,
    state: Matrix,
    next_state: Matrix,
}

impl GameOfLifeVisualizer {
    /// MatrixVisualizerインスタンスを生成する
    ///
    /// # Arguments
    /// * `title` - ウィンドウに表示するタイトル
    /// * `vertex_glsl_path` - バーテックスシェーダーのファイルを格納しているpath
    /// * `grafic_glsl_path` - グラフィックシェーダーのファイルを格納しているpath
    /// * `history_size` - 何個前の配列まで画面に表示するか
    /// * `initial_state` - 表示される内容の初期値
    pub fn new(
        title: &str,
        vertex_glsl_path: &str,
        faragment_glsl_path: &str,
    ) -> Result<GameOfLifeVisualizer, failure::Error> {
        let matrix_visualizer = MatrixVisualizer::new(title, vertex_glsl_path, faragment_glsl_path)?;
        let state = [[0; WIDTH]; HEIGHT];
        let next_state = [[0; WIDTH]; HEIGHT];
        let mut rng = thread_rng();
        let n: u8 = rng.gen_range(0, 2);
        let mut state: Vec<Vec<u8>> = Vec::with_capacity(HEIGHT);
        for i in 0..HEIGHT {
            let mut inner: Vec<u8> = Vec::new();
            for j in 0..WIDTH {
                inner.push(rng.gen_range(0, 2));
            }
            state.push(inner);
        }
        // let state = vec![vec![0; WIDTH]; HEIGHT];
        let next_state = vec![vec![0; WIDTH]; HEIGHT];
        Ok(GameOfLifeVisualizer {
            matrix_visualizer: matrix_visualizer,
            state: state,
            next_state: next_state,
        })
    }

    /// メインループ
    ///
    /// # Arguments
    /// * `initail_state` - 初期状態
    /// * `rule` - ウルフラムのルールコーディングの数字
    /// * `unpdate_fn` - 描画する状態をどのように変更するかの関数
    pub fn draw_loop<F>(mut self, mut update_fn: F) -> Result<(), failure::Error>
    where
        F: FnMut(&mut Matrix, &mut Matrix, usize, usize),
    {
        let mut window_status = WindowStatus::Open;

        // main loop
        loop {
            if window_status == WindowStatus::Close {
                break;
            }

            update_fn(&mut self.state, &mut self.next_state, HEIGHT, WIDTH);
            // self.update_matrix(&initial_state.0);
            // let hoge = self.state.iter().flatten().collect::<Vec<_>>();
            let hoge = self.state.iter().flatten().map(|e| 1.0 - *e as f32).collect::<Vec<_>>();
            self.matrix_visualizer
                // .draw(&ArrayView::from_shape((WIDTH, HEIGHT), &hoge).unwrap())?;
                .draw(&Array::from_shape_vec((WIDTH, HEIGHT), hoge).unwrap())?;
            window_status = self.matrix_visualizer.hadling_event();
        }
        Ok(())
    }

    fn update_matrix(&mut self) {
        // fn update_matrix(&mut self, matrix: &[[usize; 20]; 20]) {
        // let a = arr2(matrix);
        // self.matrix
        //     .slice_mut(s![self.time_index, ..])
        //     .assign(&(1.0 - array.map(|e| *e as f32)));
        // self.time_index = (self.time_index + 1) % self.history_size;
    }
    pub fn draw_loop_parallel_by_channel<F>(mut self, mut update_fn: F) -> Result<(), failure::Error>
    where
        F: FnMut(&Matrix, usize, usize) -> Matrix,
    {
        use std::borrow::Borrow;
        let mut window_status = WindowStatus::Open;
        use std::sync::mpsc::channel;
        use std::thread::spawn;
        let (sender, receiver) = channel();
        let mut state = self.state.clone();
        let mut n = 0;
        let handle = spawn(move || loop {
            let mut new_state = game_of_life_by_rayon(&state, HEIGHT, WIDTH);
            // println!("send: {}", n);
            sender.send((n, new_state.clone()));
            mem::swap(&mut new_state, &mut state);
            n = (n + 1) % 100000;
        });

        // main loop
        use std::{thread, time};
        let ten_millis = time::Duration::from_millis(100);

        for (i, state) in receiver {
            // println!("draw: {}", i);
            // thread::sleep(ten_millis);
            let hoge = state.iter().flatten().map(|e| 1.0 - *e as f32).collect::<Vec<_>>();
            self.matrix_visualizer
                .draw(&Array::from_shape_vec((WIDTH, HEIGHT), hoge).unwrap())?;
        }
        Ok(())
    }
}

pub struct GameOfLifeVisualizerParallel {
    matrix_visualizer: MatrixVisualizer,
    state: Arc<Matrix>,
    next_state: Arc<Matrix>,
}

impl GameOfLifeVisualizerParallel {
    pub fn new(
        title: &str,
        vertex_glsl_path: &str,
        faragment_glsl_path: &str,
    ) -> Result<GameOfLifeVisualizerParallel, failure::Error> {
        let matrix_visualizer = MatrixVisualizer::new(title, vertex_glsl_path, faragment_glsl_path)?;
        let state = [[0; WIDTH]; HEIGHT];
        let next_state = [[0; WIDTH]; HEIGHT];
        let mut rng = thread_rng();
        let n: u8 = rng.gen_range(0, 2);
        let mut state: Vec<Vec<u8>> = Vec::with_capacity(HEIGHT);
        for i in 0..HEIGHT {
            let mut inner: Vec<u8> = Vec::new();
            for j in 0..WIDTH {
                inner.push(rng.gen_range(0, 2));
            }
            state.push(inner);
        }
        // let state = vec![vec![0; WIDTH]; HEIGHT];
        let next_state = vec![vec![0; WIDTH]; HEIGHT];
        Ok(GameOfLifeVisualizerParallel {
            matrix_visualizer: matrix_visualizer,
            state: Arc::new(state),
            next_state: Arc::new(next_state),
        })
    }
    pub fn draw_loop_parallel<F>(mut self, mut update_fn: F) -> Result<(), failure::Error>
    where
        F: FnMut(Arc<Matrix>, usize, usize) -> Matrix,
    {
        let mut window_status = WindowStatus::Open;

        // main loop
        loop {
            if window_status == WindowStatus::Close {
                break;
            }

            self.next_state = Arc::new(update_fn(self.state.clone(), HEIGHT, WIDTH));
            // self.update_matrix(&initial_state.0);
            // let hoge = self.state.iter().flatten().collect::<Vec<_>>();

            mem::swap(&mut self.state, &mut self.next_state);
            let hoge = self.state.iter().flatten().map(|e| 1.0 - *e as f32).collect::<Vec<_>>();
            self.matrix_visualizer
                // .draw(&ArrayView::from_shape((WIDTH, HEIGHT), &hoge).unwrap())?;
                .draw(&Array::from_shape_vec((WIDTH, HEIGHT), hoge).unwrap())?;
            window_status = self.matrix_visualizer.hadling_event();
        }
        Ok(())
    }

    pub fn draw_loop_parallel_by_rayon<F>(mut self, mut update_fn: F) -> Result<(), failure::Error>
    where
        F: FnMut(&Matrix, usize, usize) -> Matrix,
    {
        use std::borrow::Borrow;
        let mut window_status = WindowStatus::Open;

        // main loop
        loop {
            if window_status == WindowStatus::Close {
                break;
            }

            self.next_state = Arc::new(update_fn((&self.state).borrow(), HEIGHT, WIDTH));
            // self.update_matrix(&initial_state.0);
            // let hoge = self.state.iter().flatten().collect::<Vec<_>>();

            mem::swap(&mut self.state, &mut self.next_state);
            let hoge = self.state.iter().flatten().map(|e| 1.0 - *e as f32).collect::<Vec<_>>();
            self.matrix_visualizer
                // .draw(&ArrayView::from_shape((WIDTH, HEIGHT), &hoge).unwrap())?;
                .draw(&Array::from_shape_vec((WIDTH, HEIGHT), hoge).unwrap())?;
            window_status = self.matrix_visualizer.hadling_event();
        }
        Ok(())
    }
}
