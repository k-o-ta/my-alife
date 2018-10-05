use ndarray::Array;
use ndarray::Array2;
use ndarray_rand::RandomExt;
use ndarray_rand::F32;
use num::cast as num_cast;
use num::Integer;
use num_traits::cast as num_trait_cast;
use rand::distributions::Range;
use std::ops::AddAssign;
use visualizer::matrix_visualizer::Matrix;

// simulation parameter
const DX: f32 = 0.01;
const DT: u32 = 1;
const VISUALIZATION_STEP: usize = 8;
const SPACE_GRID_SIZE: usize = 256;

// model parameter
const DU: f32 = 2e-5;
const DV: f32 = 1e-5;

/// Matrixの初期状態の一例
pub fn initial_matrix() -> DoubleMatrix<f32> {
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

    DoubleMatrix { left: u, right: v }
}

/// 与えられたMatrixを拡散させる。ラプラシアンを使って計算する
///
/// # Arguments
/// * `uv` - 拡散するもとのやつ
/// * `f` - 拡散するときの変数1
/// * `k` - 拡散するときの変数2
///
/// # Example
/// ```
/// extern crate ndarray;
/// extern crate my_alife;
///
/// use ndarray::Array2;
/// use my_alife::algorithm::gray_scott::{laplacian, DoubleMatrix, initial_matrix};
///
/// let mut state = initial_matrix();
/// laplacian(&mut state, 0.4, 0.6);
/// ```
pub fn laplacian(uv: &mut DoubleMatrix<f32>, f: f32, k: f32) {
    let u: &mut Matrix<f32> = &mut uv.left;
    let v: &mut Matrix<f32> = &mut uv.right;
    for _ in 0..VISUALIZATION_STEP {
        // ラプラシアンの計算
        let laplacian_u =
            (roll(&u, 1, false) + roll(&u, -1, false) + roll(&u, 1, true) + roll(&u, -1, true) - &*u * 4.0) / (DX * DX);
        let laplacian_v =
            (roll(&v, 1, false) + roll(&v, -1, false) + roll(&v, 1, true) + roll(&v, -1, true) - &*v * 4.0) / (DX * DX);

        // Gray-Scottモデル方程式
        let dudt = (laplacian_u * DU) - (&*u * &*v * &*v) + f * (1.0 - &*u);
        let dvdt = (laplacian_v * DV) + (&*u * &*v * &*v) - (f + k) * &*v;

        *u = (DT as f32 * dudt) + &*u;
        *v = (DT as f32 * dvdt) + &*v;
    }
}

pub struct DoubleMatrix<T> {
    left: Matrix<T>,
    right: Matrix<T>,
}

impl<T> AsRef<Matrix<T>> for DoubleMatrix<T> {
    fn as_ref(&self) -> &Matrix<T> {
        &self.left
    }
}

fn roll<A, T>(a: &Matrix<A>, shift: T, axis: bool) -> Matrix<A>
where
    A: Copy,
    T: Integer + num_trait_cast::NumCast,
{
    let shift: i32 = num_cast(shift).unwrap();
    let mut rotated = unsafe { Array2::uninitialized(a.dim()) };
    if axis {
        rotated.slice_mut(s![.., ..shift]).assign(&a.slice(s![.., -shift..]));
        rotated.slice_mut(s![.., shift..]).assign(&a.slice(s![.., ..-shift]));
    } else {
        rotated.slice_mut(s![..shift, ..]).assign(&a.slice(s![-shift.., ..]));
        rotated.slice_mut(s![shift.., ..]).assign(&a.slice(s![..-shift, ..]));
    }
    rotated
}
