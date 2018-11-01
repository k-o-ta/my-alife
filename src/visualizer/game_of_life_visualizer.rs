use algorithm::game_of_life::game_of_life_by_rayon;
use failure;
use ndarray::prelude::*;
use rand::{thread_rng, Rng};
use std::mem;
use std::sync::Arc;
use visualizer::matrix_visualizer::MatrixVisualizer;
use visualizer::WindowStatus;
const WIDTH: usize = 50;
const HEIGHT: usize = WIDTH;

pub type Matrix = Vec<Vec<u8>>;

/// 2次元配列を用いてlife gameをvisualizeする構造体
pub struct GameOfLifeVisualizer {
    matrix_visualizer: MatrixVisualizer,
    state: Matrix,
    next_state: Matrix,
}

impl GameOfLifeVisualizer {
    /// GameOfLifeVisualizerインスタンスを生成する
    ///
    /// # Arguments
    /// * `title` - ウィンドウに表示するタイトル
    /// * `vertex_glsl_path` - バーテックスシェーダーのファイルを格納しているpath
    /// * `grafic_glsl_path` - グラフィックシェーダーのファイルを格納しているpath
    pub fn new(
        title: &str,
        vertex_glsl_path: &str,
        faragment_glsl_path: &str,
    ) -> Result<GameOfLifeVisualizer, failure::Error> {
        let matrix_visualizer = MatrixVisualizer::new(title, vertex_glsl_path, faragment_glsl_path)?;
        let mut rng = thread_rng();
        let mut state: Vec<Vec<u8>> = Vec::with_capacity(HEIGHT);
        for _i in 0..HEIGHT {
            let mut inner: Vec<u8> = Vec::new();
            for _j in 0..WIDTH {
                inner.push(rng.gen_range(0, 2));
            }
            state.push(inner);
        }
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
            let state_for_show = self.state.iter().flatten().map(|e| 1.0 - *e as f32).collect::<Vec<_>>();
            self.matrix_visualizer
                .draw(&Array::from_shape_vec((WIDTH, HEIGHT), state_for_show)?)?;
            window_status = self.matrix_visualizer.hadling_event();
        }
        Ok(())
    }
    pub fn draw_loop_parallel_by_rayon<F>(mut self, mut update_fn: F) -> Result<(), failure::Error>
    where
        F: FnMut(&Matrix, usize, usize) -> Matrix,
    {
        let mut window_status = WindowStatus::Open;

        // main loop
        loop {
            if window_status == WindowStatus::Close {
                break;
            }

            self.next_state = update_fn(&self.state, HEIGHT, WIDTH);

            mem::swap(&mut self.state, &mut self.next_state);
            let state_for_show = self.state.iter().flatten().map(|e| 1.0 - *e as f32).collect::<Vec<_>>();
            self.matrix_visualizer
                .draw(&Array::from_shape_vec((WIDTH, HEIGHT), state_for_show)?)?;
            window_status = self.matrix_visualizer.hadling_event();
        }
        Ok(())
    }

    pub fn draw_loop_parallel_by_channel(mut self) -> Result<(), failure::Error> {
        use std::sync::mpsc::channel;
        use std::thread::spawn;
        let mut window_status = WindowStatus::Open;
        let (sender, receiver) = channel();
        let mut state = self.state.clone();

        // calculation thread
        let _handle = spawn(move || loop {
            let mut new_state = game_of_life_by_rayon(&state, HEIGHT, WIDTH);
            let _result = sender.send(new_state.clone());
            mem::swap(&mut new_state, &mut state);
        });

        // main thread
        for state in receiver {
            if window_status == WindowStatus::Close {
                break;
            }
            let state_for_show = state.iter().flatten().map(|e| 1.0 - *e as f32).collect::<Vec<_>>();
            self.matrix_visualizer
                .draw(&Array::from_shape_vec((WIDTH, HEIGHT), state_for_show)?)?;
            window_status = self.matrix_visualizer.hadling_event();
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
        let mut rng = thread_rng();
        let mut state: Vec<Vec<u8>> = Vec::with_capacity(HEIGHT);
        for _i in 0..HEIGHT {
            let mut inner: Vec<u8> = Vec::new();
            for _j in 0..WIDTH {
                inner.push(rng.gen_range(0, 2));
            }
            state.push(inner);
        }
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

            mem::swap(&mut self.state, &mut self.next_state);
            let state_for_show = self.state.iter().flatten().map(|e| 1.0 - *e as f32).collect::<Vec<_>>();
            self.matrix_visualizer
                .draw(&Array::from_shape_vec((WIDTH, HEIGHT), state_for_show)?)?;
            window_status = self.matrix_visualizer.hadling_event();
        }
        Ok(())
    }
}
