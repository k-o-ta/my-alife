// mod visualizer;
extern crate gl;
extern crate glutin;
#[macro_use]
extern crate glium;
#[macro_use(s)]
extern crate ndarray;
extern crate my_alife;
extern crate ndarray_rand;
extern crate num;
extern crate num_traits;
extern crate rand;

use my_alife::visualizer;
use ndarray::prelude::*;
use ndarray::Array;
use ndarray::Array2;
use ndarray_rand::F32;
use ndarray_rand::RandomExt;
use num::Integer;
use rand::distributions::Range;
use std::ops::AddAssign;

// simulation parameter
const SPACE_GRID_SIZE: usize = 256;
const DX: f32 = 0.01;
const DT: u32 = 1;
const VISUALIZATION_STEP: usize = 8;

// model parameter
const DU: f32 = 2e-5;
const DV: f32 = 1e-5;
const F: f32 = 0.04;
const K: f32 = 0.06;

type Matrix<T> = ndarray::ArrayBase<ndarray::OwnedRepr<T>, ndarray::Dim<[usize; 2]>>;

fn main() {
    let (u, v) = make_matrix();
    visualizer::matrix_visualizer::draw((u, v), lap);
    // draw_triangle(u, v);
}

// fn make() -> visualizer::Matrix<f32> {
//     let (u, v) = make_matrix();
//     lap(u, v)
// }

// F: FnMut(&T) -> &Matrix<f32>,
fn lap(uv: &mut (Matrix<f32>, Matrix<f32>)) -> &Matrix<f32> {
    for _ in 0..VISUALIZATION_STEP {
        // let mut u = &uv.0;
        // let ref v = (*uv).1;
        // ラプラシアンの計算
        let laplacian_u = (roll(&uv.0, 1, false)
            + roll(&uv.0, -1, false)
            + roll(&uv.0, 1, true)
            + roll(&uv.0, -1, true) - &uv.0 * 4.0) / (DX * DX);
        let laplacian_v = (roll(&uv.1, 1, false)
            + roll(&uv.1, -1, false)
            + roll(&uv.1, 1, true)
            + roll(&uv.1, -1, true) - &uv.1 * 4.0) / (DX * DX);

        // Gray-Scottモデル方程式
        let dudt = (laplacian_u * DU) - (&uv.0 * &uv.1 * &uv.1) + F * (1.0 - &uv.0);
        let dvdt = (laplacian_v * DV) + (&uv.0 * &uv.1 * &uv.1) - (F + K) * &uv.1;
        uv.0 = ((DT as f32 * dudt) + &uv.0);
        uv.1 = ((DT as f32 * dvdt) + &uv.1);
        // uv.0 = (&uv.0 + (DT as f32 * dudt));
        // uv.0 = (uv.1 + (DT as f32 * dvdt));
        // u = u + (DT as f32 * dudt);
        // uv.0 = hoge;
        // let fuga = *v + (DT as f32 * dvdt);
        // uv = &(, );
    }
    &uv.0
}

fn make_texture_image<'a>(u: &Matrix<f32>) -> glium::texture::RawImage2d<'a, u8> {
    let mut texture_data = Vec::new();
    for row in u.outer_iter() {
        for e in row.iter() {
            let v = (if *e < 0.0 {
                0.0
            } else if *e > 1.0 {
                1.0
            } else {
                *e
            } * 255.0) as u8;

            texture_data.push(v);
            texture_data.push(v);
            texture_data.push(v);
            texture_data.push(v);
        }
    }
    glium::texture::RawImage2d::from_raw_rgba(texture_data, (256, 256))
}

fn draw_triangle(mut u: Matrix<f32>, mut v: Matrix<f32>) {
    use glium::{glutin, Surface};
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions((600, 600).into())
        .with_title("Gray Scott");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        a_position: [f32; 2],
        a_texcoord: [f32; 2],
    }
    implement_vertex!(Vertex, a_position, a_texcoord);

    let vertex1 = Vertex {
        a_position: [-1.0, -1.0],
        a_texcoord: [0.0, 1.0],
    };
    let vertex2 = Vertex {
        a_position: [1.0, -1.0],
        a_texcoord: [1.0, 1.0],
    };
    let vertex3 = Vertex {
        a_position: [1.0, 1.0],
        a_texcoord: [1.0, 0.0],
    };
    let vertex4 = Vertex {
        a_position: [-1.0, -1.0],
        a_texcoord: [0.0, 1.0],
    };
    let vertex5 = Vertex {
        a_position: [-1.0, 1.0],
        a_texcoord: [0.0, 0.0],
    };
    let vertex6 = Vertex {
        a_position: [1.0, 1.0],
        a_texcoord: [1.0, 0.0],
    };
    let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program0 = glium::Program::from_source(
        &display,
        include_str!("../res/shaders/matrix_visualizer_vertex.glsl"),
        include_str!("../res/shaders/matrix_visualizer_fragment.glsl"),
        None,
    );
    let program = program0.unwrap();

    let mut closed = false;
    while !closed {
        for _ in 0..VISUALIZATION_STEP {
            // ラプラシアンの計算
            let laplacian_u =
                (roll(&u, 1, false) + roll(&u, -1, false) + roll(&u, 1, true) + roll(&u, -1, true)
                    - &u * 4.0) / (DX * DX);
            let laplacian_v =
                (roll(&v, 1, false) + roll(&v, -1, false) + roll(&v, 1, true) + roll(&v, -1, true)
                    - &v * 4.0) / (DX * DX);

            // Gray-Scottモデル方程式
            let dudt = (laplacian_u * DU) - (&u * &v * &v) + F * (1.0 - &u);
            let dvdt = (laplacian_v * DV) + (&u * &v * &v) - (F + K) * &v;
            u = u + (DT as f32 * dudt);
            v = v + (DT as f32 * dvdt);
        }

        let image = make_texture_image(&u);
        let texture = glium::texture::Texture2d::new(&display, image).unwrap();
        let mut target = display.draw();
        target.clear_color(1.0, 0.0, 0.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniform! { u_texture: texture.sampled() },
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                if let glutin::WindowEvent::CloseRequested = event {
                    closed = true
                }
            }
        });
    }
}

fn make_matrix() -> (Matrix<f32>, Matrix<f32>) {
    // initialize
    let mut u = Array2::<f32>::ones((256, 256));
    let mut v = Array2::<f32>::zeros((256, 256));

    // 中央にSQUARE_SIZE四方の正方形を置く
    const SQUARE_SIZE: usize = 20;
    u.slice_mut(s![
        SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
        SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
    ]).fill(0.5);
    v.slice_mut(s![
        SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
        SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
    ]).fill(0.25);

    // 対称性を崩すため少しノイズを入れる
    let u_rand = Array::random((SPACE_GRID_SIZE, SPACE_GRID_SIZE), F32(Range::new(0., 1.))) * 0.1;
    let v_rand = Array::random((SPACE_GRID_SIZE, SPACE_GRID_SIZE), F32(Range::new(0., 1.))) * 0.1;
    u.add_assign(&u_rand);
    v.add_assign(&v_rand);

    (u, v)
}

// #[allow(clippy)]
fn roll<A, T>(a: &Matrix<A>, shift: T, axis: bool) -> Matrix<A>
where
    A: Copy,
    T: Integer + num_traits::cast::NumCast,
{
    let shift: i32 = num::cast(shift).unwrap();
    let mut rotated = unsafe { Array2::uninitialized(a.dim()) };
    if axis {
        rotated
            .slice_mut(s![.., ..shift])
            .assign(&a.slice(s![.., -shift..]));
        rotated
            .slice_mut(s![.., shift..])
            .assign(&a.slice(s![.., ..-shift]));
    } else {
        rotated
            .slice_mut(s![..shift, ..])
            .assign(&a.slice(s![-shift.., ..]));
        rotated
            .slice_mut(s![shift.., ..])
            .assign(&a.slice(s![..-shift, ..]));
    }
    rotated
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
