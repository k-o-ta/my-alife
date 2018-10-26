#[macro_use(s)]
extern crate ndarray;
use ndarray::prelude::*;
use ndarray::{arr2, Array, ShapeBuilder};

fn main() {
    println!("hello world");
    // to_matrix();
    stride();
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
