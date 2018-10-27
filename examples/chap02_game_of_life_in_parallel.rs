extern crate my_alife;

use my_alife::algorithm::game_of_life::game_of_life_in_parallel;
use my_alife::visualizer::game_of_life_visualizer::GameOfLifeVisualizerParallel;
use std::fmt::Debug;

fn main() -> Result<(), impl Debug> {
    let visualizer = GameOfLifeVisualizerParallel::new(
        "Game Of Life in parallel",
        "res/shaders/matrix_visualizer_vertex.glsl",
        "res/shaders/matrix_visualizer_fragment.glsl",
    );
    visualizer?.draw_loop_parallel(game_of_life_in_parallel)
}
