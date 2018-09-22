extern crate my_alife;

use my_alife::algorithm::gray_scott::{initial_matrix, laplacian};
use my_alife::visualizer::matrix_visualizer::MatrixVisualizer;
use std::fmt::Debug;

// model parameter
const F: f32 = 0.022;
const K: f32 = 0.051;

fn main() -> Result<(), impl Debug> {
    let (u, v) = initial_matrix();
    let matrix = MatrixVisualizer::new(
        "Gray Scott",
        "res/shaders/matrix_visualizer_vertex.glsl",
        "res/shaders/matrix_visualizer_fragment.glsl",
    );
    matrix?.draw_loop((u, v), F, K, laplacian)
}
