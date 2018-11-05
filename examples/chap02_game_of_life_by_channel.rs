extern crate my_alife;

use my_alife::visualizer::game_of_life_visualizer::GameOfLifeVisualizer;
use std::fmt::Debug;

fn main() -> Result<(), impl Debug> {
    let visualizer = GameOfLifeVisualizer::new(
        "Game Of Life in parallel",
        "res/shaders/matrix_visualizer_vertex.glsl",
        "res/shaders/matrix_visualizer_fragment.glsl",
    );
    visualizer?.draw_loop_parallel_by_channel()
}
