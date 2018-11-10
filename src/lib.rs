//! # my-alife
//! [作って動かすAlife](https://www.oreilly.co.jp/books/9784873118475/)の[コード](https://github.com/alifelab/alife_book_src)をRustで実装してみる。Rustの勉強も兼ねて過剰にRustの機能を使ったりしている
//!
//! ## GLSLについて
//! 内部でglslのコードがあるが、少なくとも[このくらい](http://nn-hokuson.hatenablog.com/entry/2016/11/07/204241)を理解しているとわかりそう  
//! もっとよく知りたい場合は[これ](http://tkengo.github.io/blog/2014/12/27/opengl-es-2-2d-knowledge-1/)も良さそう
//!
//! ## モジュール化の方針
//! パターンの生成ロジックを担当するalgorithmと描画を担当するvisualizerに分けて実装していく
//!
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
extern crate rayon;

extern crate failure;

extern crate piston_window;

/// パターン生成のアルゴリズム
pub mod algorithm;
/// 複数の描画方法をまとめたもの
pub mod visualizer;

pub mod simulator;
