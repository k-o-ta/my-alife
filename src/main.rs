extern crate my_alife;
#[macro_use(s)]
extern crate ndarray;
extern crate rand;
use my_alife::algorithm::game_of_life::game_of_life_in_parallel;
use ndarray::prelude::*;
use ndarray::{arr2, Array, ShapeBuilder};
use rand::{thread_rng, Rng};
use std::sync::Arc;

fn main() {
    println!("hello world");
    // to_matrix();
    stride();
    do_thread_handles();
}

#[allow(dead_code)]
fn roll2() {
    let a = arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    println!("{:?}", a);
    println!("{:?}", 1 - &a);

    let b = arr2(&[[2, 2, 2], [3, 3, 3], [4, 4, 4]]);
    println!("{:?}", &a * &b);

    for i in a.outer_iter() {
        for j in i.iter() {
            println!("{}", j)
        }
    }
}

fn to_matrix() {
    let mut a = arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    println!("{:?}", a);
    println!("{:?}", a.slice_mut(s![0, ..]));
    a.slice_mut(s![0, ..]).assign(&Array::from_vec(vec![0, 0, 0]));
    println!("{:?}", a);
    let mut a = arr2(&[[1, 2, 3, 4], [4, 5, 6, 4], [7, 8, 9, 4], [7, 8, 9, 4]]);
}

fn stride() {
    // let a = [[1, 2], [3, 4]].iter().flatten().collect::<Vec<u8>>();
    let a = vec![vec![1, 2], vec![3, 4]].into_iter().flatten().collect::<Vec<u8>>();
    // let a = vec![vec![1, 2], vec![3, 4]].iter().flatten().collect::<&Vec<u8>>();
    let a = [[1, 2], [3, 4]].iter().flatten().collect::<Vec<_>>();
    let c = ArrayView::from_shape((2, 2), &a);
    println!("{:?}", c);
    // println!("{:?}", a);
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let b = ArrayView::from_shape((3, 3), &v);
    println!("{:?}", b);
}

fn do_thread_handles() {
    let height = 50;
    let width = 50;
    let mut state: Vec<Vec<u8>> = Vec::with_capacity(height);
    let mut rng = thread_rng();
    for i in 0..height {
        let mut inner: Vec<u8> = Vec::new();
        for j in 0..width {
            inner.push(rng.gen_range(0, 2));
        }
        state.push(inner);
    }
    let arc = Arc::new(state);
    game_of_life_in_parallel(arc, height, width);
}
