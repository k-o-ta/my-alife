extern crate my_alife;
extern crate ndarray;
extern crate ndarray_rand;
extern crate rand;

use my_alife::algorithm::cellular_automata::cellular_automata;
use my_alife::visualizer::array_visualizer::ArrayVisualizer;
use ndarray::Array1;
use ndarray_rand::RandomExt;
use rand::distributions::Range;
use std::fmt::Debug;

fn main() -> Result<(), impl Debug> {
    let len = 600;
    let mut initial_array = Array1::<u32>::zeros(len);
    // 初期値固定
    // initial_array.slice_mut(s![len / 2]).fill(1);
    // 初期値ランダム
    initial_array.assign(&Array1::random(len, Range::new(0, 2)));
    let rule = 30;
    let next_state = Array1::<u32>::zeros(len);
    let visualizer = ArrayVisualizer::new(
        "Cellular Automata 1d",
        "res/shaders/matrix_visualizer_vertex.glsl",
        "res/shaders/matrix_visualizer_fragment.glsl",
        600,
        &initial_array,
    );
    visualizer?.draw_loop((initial_array, next_state), rule, cellular_automata)
}
