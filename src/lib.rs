extern crate gl;
extern crate glutin;
#[macro_use]
extern crate glium;
#[macro_use(s)]
extern crate ndarray;
extern crate ndarray_rand;
extern crate num;
extern crate num_traits;
extern crate rand;

extern crate failure;

/// パターン生成のアルゴリズム
pub mod algorithm;
/// 複数の描画方法をまとめたもの
pub mod visualizer;
