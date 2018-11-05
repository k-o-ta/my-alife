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
    ///
    /// # iterator
    /// 1. Vec<Vec<u8>>をVec<u8>の形にしないとndarrayの形に変換できない
    ///   * [[0,1,2],[3,4,5],[6,7,8]] -> [0,1,2,3,4,5,6,7,8,9]にする。
    ///   * from_shape_vec((3,3) [0,1,2,3,4,5,6,7,8,9])すると、3つずつ取っていく
    /// 2. stateは0だと死(白)、1だと生(黒)であるが、visualizer的には0だと黒、1だと白に表示される
    /// 反転するためにmapの中で変換している
    /// https://doc.rust-lang.org/book/second-edition/ch13-02-iterators.html
    /// ## iteratorのよくある使い方
    /// 1. iterator Traitを実装している型のデータにinto_iter(), iter(), iter_mut()を使う(それぞれmove, borrow, mutable borrowに相当する)
    ///   * iter()
    ///     * 各要素の所有権をiter内にmoveする。もとの配列は使えなくなる
    ///   * into_iter()
    ///     * iter内で使えるのは各要素の参照(最も制限が厳しい)
    ///   * iter_mut()
    ///     * 各要素のmutableな参照をiter内で使える
    /// 2. mapとかfilterとかをiteratorに使うとiteratorを返す(iterator adapter)
    ///   * [たくさんある](https://doc.rust-lang.org/std/iter/trait.Iterator.html#provided-methods)
    ///   * ここまでは遅延評価されている
    /// 3. collect()などconsume adapterを使うとiteratorを何らかの型に戻せる
    ///   * ここで評価される
    ///   * collectは何らかのiteratorを何らかのcollection型にする
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
            // collect::<HashMap<_, _>>()とか、collect::<Result<u8, _>>とか、collect::<String>とか。
            // let hoge: String = iterator.collect();みたいに変数側で指定してもよいし、
            // let hoge = iterator.collect();
            // pass_string(hoge);
            // のように型推論で型指定を省略もできる
            // ↓だとmap内の処理で新しい要素(u8)を作ってそれのvecを作っている。mapの返り値がeを参照しているとcollectが返す値もVec<&u8>みたいになるはず。
            let state_for_show = self.state.iter().flatten().map(|e| 1.0 - *e as f32).collect::<Vec<_>>();
            self.matrix_visualizer
                .draw(&Array::from_shape_vec((WIDTH, HEIGHT), state_for_show)?)?;
            window_status = self.matrix_visualizer.hadling_event();
        }
        Ok(())
    }

    /// ライフゲームの計算用のスレッドを描画用のスレッド(main thread)と分ける
    /// * 計算用スレッドでループを回し、1ループ毎にメインスレッドに計算結果を送る
    /// * thread間通信には一方通行のchannelを用いる
    /// * Producer(計算スレッド)-Consumer(描画スレッド)パターン
    ///   * https://doc.rust-lang.org/std/sync/mpsc/index.html
    ///   * Producerは複数いても良いがConsumerは一人のみなのでMultiProducerSingleConsumer(mpsc)
    pub fn draw_loop_parallel_by_channel(mut self) -> Result<(), failure::Error> {
        use std::sync::mpsc::channel;
        use std::thread::spawn;
        let mut window_status = WindowStatus::Open;
        let (sender, receiver) = channel();
        let mut state = self.state.clone();

        // calculation thread
        let _handle = spawn(move || loop {
            let mut new_state = game_of_life_by_rayon(&state, HEIGHT, WIDTH);
            // channelにデータを送っている
            // データを送るときは所有権ごと送ってしまうので、cloneしておかないと次回のloopのときにstateが使えなくなる
            // 所有権ごと送ることでthread safeを実現している(writableなユーザーが同時に一人しか存在できない)
            let _result = sender.send(new_state.clone());
            mem::swap(&mut new_state, &mut state);
        });

        // main thread
        // channelを受信するまでblockingしている
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
