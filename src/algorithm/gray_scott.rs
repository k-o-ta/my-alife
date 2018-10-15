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
pub fn initial_matrix() -> (Matrix<f32>, Matrix<f32>) {
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

/// 与えられたMatrixを拡散させる。ラプラシアンを使って計算する
///
/// # Arguments
/// * `uv` - 拡散するもとのやつ
/// * `f` - 拡散するときの変数1
/// * `k` - 拡散するときの変数2
///
/// ## mutable reference
/// `uv`(`&mut borrow`)はmutableな参照である
/// `borrow`したい(`move`したくない)が、変更もしたい、という場合に使う。
/// `borrow`されたデータは必ず変更されると思って良い。  
/// `borrow`したデータを変更できるが、複数ヶ所から同時に`borrow`できなくなる。
/// ~~~
/// extern crate ndarray;
/// extern crate my_alife;
///
/// use my_alife::visualizer::matrix_visualizer::Matrix;
/// use my_alife::algorithm::gray_scott::laplacian;
/// use ndarray::Array2;
///
/// let mut state = (Array2::<f32>::ones((256, 256)),Array2::<f32>::ones((256, 256)));
/// laplacian(&mut state, 0.5, 0.5); // &mutな参照を渡している
/// println!("{:?}", state);         // stateがまだ使える
/// ~~~
///
/// [資料](https://doc.rust-lang.org/book/2018-edition/ch04-02-references-and-borrowing.html#mutable-references)
/// [日本語訳](https://github.com/hazama-yuinyan/book/blob/master/second-edition/src/ch04-02-references-and-borrowing.md#%E5%8F%AF%E5%A4%89%E3%81%AA%E5%8F%82%E7%85%A7)
///
/// # Example
/// ```
/// extern crate ndarray;
/// extern crate my_alife;
///
/// use ndarray::Array2;
/// use my_alife::algorithm::gray_scott::laplacian;
///
/// let mut state = (Array2::<f32>::ones((256, 256)), Array2::<f32>::ones((256, 256)));
/// let matrix = laplacian(&mut state, 0.4, 0.6);
/// ```
pub fn laplacian(uv: &mut (Matrix<f32>, Matrix<f32>), f: f32, k: f32) {
    let u: &mut Matrix<f32> = &mut uv.0;
    let v: &mut Matrix<f32> = &mut uv.1;
    for _ in 0..VISUALIZATION_STEP {
        // ラプラシアンの計算
        let laplacian_u: Matrix<f32> =
            (roll(&u, 1, false) + roll(&u, -1, false) + roll(&u, 1, true) + roll(&u, -1, true) - &*u * 4.0) / (DX * DX);
        let laplacian_v =
            (roll(&v, 1, false) + roll(&v, -1, false) + roll(&v, 1, true) + roll(&v, -1, true) - &*v * 4.0) / (DX * DX);

        // Gray-Scottモデル方程式
        let dudt: Matrix<f32> = (laplacian_u * DU) - (&*u * &*v * &*v) + f * (1.0 - &*u);
        let dvdt = (laplacian_v * DV) + (&*u * &*v * &*v) - (f + k) * &*v;

        *u = (DT as f32 * dudt) + &*u;
        *v = (DT as f32 * dvdt) + &*v;
    }
}

/// lifetimeパラメーター説明用に作った関数
/// # Arguments
/// * `u` - 拡散するもとのやつ
/// * `v` - 拡散するもとのやつ
/// * `f` - 拡散するときの変数1
/// * `k` - 拡散するときの変数2
///
/// ## lifetime
/// lifetimeパラメーター `'a`が存在する。
/// ### lifetimeとは
/// * lifetimeとは __変数の生存期間__ である
///   * 変数にもownerであるものと参照であるものが存在するが、lifetimeで話題になるのは専ら参照のそれ
/// * 全ての参照にはlifetimeが存在する。しかし、多くのケースでは推論可能なので省略されている
/// * データは参照より長生きしなければならない。データの生存期間 > lifetimeになる
/// ### なんでlifetimeが必要か
/// 1. 以下の例では返り値の`&Matrix`は`u`か`v`と同じデータを必ず参照している
///     * u, v以外の関数内部で生成されたデータを参照している可能性は?
///         * そのデータは関数のスコープ内で消えるので、返り値から参照できない
/// 2. 呼び出し側からは返り値がu,vどちらのデータを参照しているかわからない。
///     * どちらの引数を参照しているかわからないと困る理由は、関数の呼び出し側で実引数のデータが返り値(参照)より長く生きているかを確認できなくなるからである
///
///
/// つまり、 __lifetimeパラメーターは呼び出し側のためのもの__ なのである。  
/// たとえば、以下のlifetimeを持つが、`laplacian_ref`の返り値が`u`への参照ならコンパイルが通るが、`v`への参照の場合、実体`v`が先に消えるので`matrix`は参照先が存在しなくなり、コンパイルが通らない。しかし、呼び出し側では`laplacian_ref`が`v`と`u`のどちらへの参照を返すかわからないのである。そのため関数のsignatureにlifetimeをつけてわかるようにしている。lifetimeパラメーターは利用者側にとって、渡す変数と返ってくる参照の関係を示したもの(関数のbodyにとってはどうでも良い)
/// 返り値がvの場合、どちらを参照するかが実行時に決まる場合の説明をする
///
/// ~~~
/// extern crate ndarray;
/// extern crate my_alife;
///
/// use my_alife::visualizer::matrix_visualizer::Matrix;
/// use my_alife::algorithm::gray_scott::laplacian_ref;
/// use ndarray::Array2;
///
/// let mut u = Array2::<f32>::ones((256, 256));          // ----┐
/// let matrix;                                           // ---┐|
/// {                                                     //    ||
///   let mut v = Array2::<f32>::zeros((256, 256));       // --┐||
///   matrix = laplacian_ref(&mut u, &mut v, 0.5, 0.5);   //   |||
///   //       lifetime 'b     <-------------------------------┘||
/// }                                                     //    ||
/// //         matrix lifetime <--------------------------------┘|
/// //         lifetime 'a     <---------------------------------┘
/// ~~~
///
/// [参考](https://doc.rust-lang.org/book/2018-edition/ch10-03-lifetime-syntax.html)
/// [日本語訳](https://github.com/hazama-yuinyan/book/blob/master/second-edition/src/ch10-03-lifetime-syntax.md)
pub fn laplacian_ref<'a, 'b>(u: &'a mut Matrix<f32>, v: &'b mut Matrix<f32>, f: f32, k: f32) -> &'a Matrix<f32> {
    for _ in 0..VISUALIZATION_STEP {
        // ラプラシアンの計算
        let laplacian_u: Matrix<f32> =
            (roll(&u, 1, false) + roll(&u, -1, false) + roll(&u, 1, true) + roll(&u, -1, true) - &*u * 4.0) / (DX * DX);
        let laplacian_v =
            (roll(&v, 1, false) + roll(&v, -1, false) + roll(&v, 1, true) + roll(&v, -1, true) - &*v * 4.0) / (DX * DX);

        // Gray-Scottモデル方程式
        let dudt: Matrix<f32> = (laplacian_u * DU) - (&*u * &*v * &*v) + f * (1.0 - &*u);
        let dvdt = (laplacian_v * DV) + (&*u * &*v * &*v) - (f + k) * &*v;

        *u = (DT as f32 * dudt) + &*u;
        *v = (DT as f32 * dvdt) + &*v;
    }
    u
    // use rand::distributions::IndependentSample;
    // use rand::thread_rng;
    // let mut rng = thread_rng();
    // if Range::new(0, 1).ind_sample(&mut rng) == 0 {
    //     u
    // } else {
    //     v
    // }
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
