pub mod matrix_visualizer;

use ndarray::{ArrayBase, Dim, OwnedRepr};

trait Visualizer {
    fn draw2<T, F>(self, T, F)
    where
        F: FnMut(&mut T) -> &Texturize<T>;
}

pub type Matrix<T> = ArrayBase<OwnedRepr<T>, Dim<[usize; 2]>>;

trait Texturize<T> {
    fn to_2d(self) -> Matrix<T>;
}
