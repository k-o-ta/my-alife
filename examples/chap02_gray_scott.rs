// mod visualizer;
extern crate gl;
extern crate glutin;
#[macro_use]
extern crate glium;
#[macro_use(s)]
extern crate ndarray;
extern crate my_alife;
extern crate ndarray_rand;
extern crate num;
extern crate num_traits;
extern crate rand;

use my_alife::visualizer;
use ndarray::prelude::*;
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

type Matrix<T> = ndarray::ArrayBase<ndarray::OwnedRepr<T>, ndarray::Dim<[usize; 2]>>;

fn main() {
    let (u, v) = make_matrix();
    visualizer::matrix_visualizer::draw((u, v), lap);
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
    for _ in 0..VISUALIZATION_STEP {
        // let mut u = &uv.0;
        // let ref v = (*uv).1;
        // ラプラシアンの計算
        let laplacian_u = (roll(&uv.0, 1, false)
            + roll(&uv.0, -1, false)
            + roll(&uv.0, 1, true)
            + roll(&uv.0, -1, true) - &uv.0 * 4.0) / (DX * DX);
        let laplacian_v = (roll(&uv.1, 1, false)
            + roll(&uv.1, -1, false)
            + roll(&uv.1, 1, true)
            + roll(&uv.1, -1, true) - &uv.1 * 4.0) / (DX * DX);

        // Gray-Scottモデル方程式
        let dudt = (laplacian_u * DU) - (&uv.0 * &uv.1 * &uv.1) + F * (1.0 - &uv.0);
        let dvdt = (laplacian_v * DV) + (&uv.0 * &uv.1 * &uv.1) - (F + K) * &uv.1;
        uv.0 = ((DT as f32 * dudt) + &uv.0);
        uv.1 = ((DT as f32 * dvdt) + &uv.1);
        // uv.0 = (&uv.0 + (DT as f32 * dudt));
        // uv.0 = (uv.1 + (DT as f32 * dvdt));
        // u = u + (DT as f32 * dudt);
        // uv.0 = hoge;
        // let fuga = *v + (DT as f32 * dvdt);
        // uv = &(, );
    }
    &uv.0
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
