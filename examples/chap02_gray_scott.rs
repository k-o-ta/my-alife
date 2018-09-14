extern crate gl;
extern crate glium;
extern crate glutin;
#[macro_use(s)]
extern crate ndarray;
extern crate my_alife;
extern crate ndarray_rand;
extern crate num;
extern crate num_traits;
extern crate rand;

use my_alife::visualizer::matrix_visualizer::{Matrix, MatrixVisualizer};
use ndarray::Array;
use ndarray::Array2;
use ndarray_rand::F32;
use ndarray_rand::RandomExt;
use num::Integer;
use rand::distributions::Range;
use std::ops::AddAssign;

// simulation parameter
const SPACE_GRID_SIZE: usize = 256;
const DX: f32 = 0.01;
const DT: u32 = 1;
const VISUALIZATION_STEP: usize = 8;

// model parameter
const DU: f32 = 2e-5;
const DV: f32 = 1e-5;
const F: f32 = 0.04;
const K: f32 = 0.06;

fn main() {
    let (u, v) = make_matrix();
    let matrix = MatrixVisualizer::new(
        "Gray Scott",
        "res/shaders/matrix_visualizer_vertex.glsl".to_string(),
        "res/shaders/matrix_visualizer_fragment.glsl".to_string(),
    );
    matrix.unwrap().draw((u, v), lap);
}

fn make_matrix() -> (Matrix<f32>, Matrix<f32>) {
    // initialize
    let mut u = Array2::<f32>::ones((256, 256));
    let mut v = Array2::<f32>::zeros((256, 256));

    // 中央にSQUARE_SIZE四方の正方形を置く
    const SQUARE_SIZE: usize = 20;
    u.slice_mut(s![
        SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
        SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
    ]).fill(0.5);
    v.slice_mut(s![
        SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
        SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
    ]).fill(0.25);

    // 対称性を崩すため少しノイズを入れる
    let u_rand = Array::random((SPACE_GRID_SIZE, SPACE_GRID_SIZE), F32(Range::new(0., 1.))) * 0.1;
    let v_rand = Array::random((SPACE_GRID_SIZE, SPACE_GRID_SIZE), F32(Range::new(0., 1.))) * 0.1;
    u.add_assign(&u_rand);
    v.add_assign(&v_rand);

    (u, v)
}

fn lap(uv: &mut (Matrix<f32>, Matrix<f32>)) -> &Matrix<f32> {
    let u: &mut Matrix<f32> = &mut uv.0;
    let v: &mut Matrix<f32> = &mut uv.1;
    for _ in 0..VISUALIZATION_STEP {
        // ラプラシアンの計算
        let laplacian_u =
            (roll(&u, 1, false) + roll(&u, -1, false) + roll(&u, 1, true) + roll(&u, -1, true)
                - &*u * 4.0) / (DX * DX);
        let laplacian_v =
            (roll(&v, 1, false) + roll(&v, -1, false) + roll(&v, 1, true) + roll(&v, -1, true)
                - &*v * 4.0) / (DX * DX);

        // Gray-Scottモデル方程式
        let dudt = (laplacian_u * DU) - (&*u * &*v * &*v) + F * (1.0 - &*u);
        let dvdt = (laplacian_v * DV) + (&*u * &*v * &*v) - (F + K) * &*v;

        *u = (DT as f32 * dudt) + &*u;
        *v = (DT as f32 * dvdt) + &*v;
    }
    u
}

// #[allow(clippy)]
fn roll<A, T>(a: &Matrix<A>, shift: T, axis: bool) -> Matrix<A>
where
    A: Copy,
    T: Integer + num_traits::cast::NumCast,
{
    let shift: i32 = num::cast(shift).unwrap();
    let mut rotated = unsafe { Array2::uninitialized(a.dim()) };
    if axis {
        rotated
            .slice_mut(s![.., ..shift])
            .assign(&a.slice(s![.., -shift..]));
        rotated
            .slice_mut(s![.., shift..])
            .assign(&a.slice(s![.., ..-shift]));
    } else {
        rotated
            .slice_mut(s![..shift, ..])
            .assign(&a.slice(s![-shift.., ..]));
        rotated
            .slice_mut(s![shift.., ..])
            .assign(&a.slice(s![..-shift, ..]));
    }
    rotated
}
