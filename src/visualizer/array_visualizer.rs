use failure;
use ndarray::{Array1, Array2};
use visualizer::matrix_visualizer::{Matrix, MatrixVisualizer};
use visualizer::WindowStatus;

/// 1次元配列を用いてvisualizeする構造体
/// 内部的に1次元配列を2次元配列(Matrix)に変換する
pub struct ArrayVisualizer {
    matrix_visualizer: MatrixVisualizer,
    history_size: usize,
    time_index: usize,
    matrix: Matrix<f32>,
}

impl ArrayVisualizer {
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
        history_size: usize,
        initial_state: &Array1<u32>,
    ) -> Result<ArrayVisualizer, failure::Error> {
        let matrix_visualizer = MatrixVisualizer::new(title, vertex_glsl_path, faragment_glsl_path)?;
        let matrix = Array2::<f32>::zeros((history_size, initial_state.len()));
        Ok(ArrayVisualizer {
            matrix_visualizer: matrix_visualizer,
            history_size: history_size,
            time_index: 0,
            matrix: matrix,
        })
    }

    /// メインループ
    ///
    /// # Arguments
    /// * `initail_state` - 初期状態
    /// * `rule` - ウルフラムのルールコーディングの数字
    /// * `unpdate_fn` - 描画する状態をどのように変更するかの関数
    pub fn draw_loop<F>(
        mut self,
        mut initial_state: (Array1<u32>, Array1<u32>),
        rule: u8,
        mut update_fn: F,
    ) -> Result<(), failure::Error>
    where
        F: FnMut(&mut (Array1<u32>, Array1<u32>), u8, usize),
    {
        let mut window_status = WindowStatus::Open;

        // main loop
        loop {
            if window_status == WindowStatus::Close {
                break;
            }

            update_fn(&mut initial_state, rule, self.history_size);
            self.update_matrix(&initial_state.0);
            self.matrix_visualizer.draw(&self.matrix)?;
            window_status = self.matrix_visualizer.hadling_event();
        }
        Ok(())
    }

    fn update_matrix(&mut self, array: &Array1<u32>) {
        self.matrix
            .slice_mut(s![self.time_index, ..])
            .assign(&(1.0 - array.map(|e| *e as f32)));
        self.time_index = (self.time_index + 1) % self.history_size;
    }
}
