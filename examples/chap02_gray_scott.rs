extern crate my_alife;

use my_alife::algorithm::gray_scott::{initial_matrix, laplacian, laplacian2};
use my_alife::visualizer::matrix_visualizer::MatrixVisualizer;
use std::fmt::Debug;

// model parameter
const F: f32 = 0.04;
const K: f32 = 0.06;

fn main() -> Result<(), impl Debug> {
    // let (u, v) = initial_matrix();
    let tuple = initial_matrix();
    let matrix = MatrixVisualizer::new(
        "Gray Scott",
        "res/shaders/matrix_visualizer_vertex.glsl",
        "res/shaders/matrix_visualizer_fragment.glsl",
    );
    matrix?.draw_loop(tuple, F, K, laplacian2)
}
