/// 直交座標系(XY座標系)を用いてvisualizeするためのモジュール
pub mod matrix_visualizer;

/// 一次元配列を用いてvisualizeするためのモジュール
pub mod array_visualizer;

/// windowの状態
#[derive(PartialEq)]
pub enum WindowStatus {
    /// 開いている
    Open,
    /// 閉じている
    Close,
}
