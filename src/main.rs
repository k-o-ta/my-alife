extern crate gl;
extern crate glutin;
#[macro_use]
extern crate glium;
#[macro_use(s)]
extern crate ndarray;
extern crate ndarray_rand;
extern crate rand;

// use glutin::dpi::*;
// use glutin::GlContext;
use ndarray::prelude::*;
use ndarray::Array;
use ndarray::Array2;
use ndarray_rand::F32;
use ndarray_rand::RandomExt;
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

fn main() {
    let (u, v) = make_matrix();
    // println!("{:?}", u);
    draw_triangle(u);
    // // initialize
    // let mut u = Array2::<f32>::ones((256, 256));
    // let mut v = Array2::<f32>::zeros((256, 256));
    // const SQUARE_SIZE: usize = 20;
    // u.slice_mut(s![
    //     SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
    //     SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2
    // ]).fill(0.5);
    // v.slice_mut(s![
    //     SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
    //     SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2
    // ]).fill(0.25);
    // let u_rand = Array::random((SPACE_GRID_SIZE, SPACE_GRID_SIZE), F32(Range::new(0., 1.))) * 0.1;
    // let v_rand = Array::random((SPACE_GRID_SIZE, SPACE_GRID_SIZE), F32(Range::new(0., 1.))) * 0.1;
    // u.add_assign(&u_rand);
    // v.add_assign(&v_rand);
    // println!("{:?}", v);
    //
    // let mut events_loop = glutin::EventsLoop::new();
    // let window = glutin::WindowBuilder::new()
    //     .with_title("chap2")
    //     .with_dimensions(LogicalSize::new(600.0, 600.0));
    // let context = glutin::ContextBuilder::new();
    // let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
    //
    // unsafe {
    //     gl_window.make_current().unwrap();
    // }
    //
    // unsafe {
    //     gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    //     gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    // }
    //
    // // let mut running = true;
    // let mut running = false;
    // while running {
    //     events_loop.poll_events(|event| match event {
    //         glutin::Event::WindowEvent { event, .. } => match event {
    //             glutin::WindowEvent::CloseRequested => running = false,
    //             glutin::WindowEvent::Resized(logical_size) => {}
    //             _ => {}
    //         },
    //         _ => {}
    //     });
    //     unsafe {
    //         gl::Clear(gl::COLOR_BUFFER_BIT);
    //     }
    //     gl_window.swap_buffers().unwrap();
    // }
}

fn draw_triangle(u: ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 2]>>) {
    use glium::{glutin, Surface};
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        a_position: [f32; 2],
    }
    implement_vertex!(Vertex, a_position);

    #[derive(Copy, Clone)]
    struct ATexcoord {
        a_texcoord: [f32; 2],
    }
    implement_vertex!(ATexcoord, a_texcoord);

    #[derive(Copy, Clone)]
    struct VTexcoord {
        v_texcoord: [f32; 2],
    }
    implement_vertex!(VTexcoord, v_texcoord);

    let vertex1 = Vertex {
        a_position: [-1.0, -1.0],
    };
    let vertex2 = Vertex {
        a_position: [-1.0, 1.0],
    };
    let vertex3 = Vertex {
        a_position: [1.0, -1.0],
    };
    let vertex4 = Vertex {
        a_position: [1.0, 1.0],
    };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let a_texcoord1 = ATexcoord {
        a_texcoord: [0.0, 1.0],
    };
    let a_texcoord2 = ATexcoord {
        a_texcoord: [0.0, 0.0],
    };
    let a_texcoord3 = ATexcoord {
        a_texcoord: [1.0, 1.0],
    };
    let a_texcoord4 = ATexcoord {
        a_texcoord: [1.0, 0.0],
    };
    let a_shape = vec![a_texcoord1, a_texcoord2, a_texcoord3, a_texcoord4];
    let a_texcoord_buffer = glium::VertexBuffer::new(&display, &a_shape).unwrap();

    // let v_texcoord1 = VTexcoord {
    //     v_texcoord: [-0.5, -0.5],
    // };
    // let v_texcoord2 = VTexcoord {
    //     v_texcoord: [0.0, 0.5],
    // };
    // let v_texcoord3 = VTexcoord {
    //     v_texcoord: [0.5, 0.25],
    // };
    // let v_texcoord4 = VTexcoord {
    //     v_texcoord: [1.0, 0.25],
    // };
    // let v_shape = vec![v_texcoord1, v_texcoord2, v_texcoord3, v_texcoord4];
    // let v_texcoord_buffer = glium::VertexBuffer::new(&display, &v_shape).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = glium::Program::from_source(
        &display,
        include_str!("../res/shaders/matrix_visualizer_vertex.glsl"),
        include_str!("../res/shaders/matrix_visualizer_fragment.glsl"),
        None,
    ).unwrap();

    // let texture_data = vec![vec![(0u8, 0, 0), (255, 255, 255)]];
    // let texture_data = vec![vec![0.0]];
    // let texture_data2: Vec<Vec<f32>> = u.into_iter().map(|d| d.clone().to_vec()).collect();
    // println!("#{:?}", u.row(1).to_vec());
    let vector = u.iter().map(|d| d).collect::<Vec<&f32>>();
    let mut texture_data: Vec<Vec<f32>> = Vec::new();
    for i in 0..256 {
        let mut inner_vec: Vec<f32> = Vec::new();
        for j in 0..256 {
            let mut v = vector[i + j].clone();
            if v < 0.0 {
                v = 0.0;
            } else if v > 1.0 {
                v = 1.0;
            }
            v = v / 255.0;

            // inner_vec.push(vector[i + j].clone())
            inner_vec.push(v);
        }
        texture_data.push(inner_vec);
    }
    let texture_data = vec![vec![0.0]];

    // let texture_data = [[1.0; 256]; 256];
    // let texture_data = u;
    let texture = glium::texture::Texture2d::new(&display, texture_data).unwrap();

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target
            .draw(
                // (&vertex_buffer, &a_texcoord_buffer, &v_texcoord_buffer),
                (&vertex_buffer, &a_texcoord_buffer),
                &indices,
                &program,
                // &glium::uniforms::EmptyUniforms,
                // &uniform! {matrix: [0.0, 0.0, 0.0, 0.0]},
                &uniform! {
                // u_texture: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                u_texture: texture.sampled()
                               },
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                _ => (),
            },
            _ => (),
        });
    }
}

fn make_matrix() -> (
    ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 2]>>,
    ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 2]>>,
) {
    // initialize
    let mut u = Array2::<f32>::ones((256, 256));
    let mut v = Array2::<f32>::zeros((256, 256));
    const SQUARE_SIZE: usize = 20;
    u.slice_mut(s![
        SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
        SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2
    ]).fill(0.5);
    v.slice_mut(s![
        SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
        SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2
    ]).fill(0.25);
    let u_rand = Array::random((SPACE_GRID_SIZE, SPACE_GRID_SIZE), F32(Range::new(0., 1.))) * 0.1;
    let v_rand = Array::random((SPACE_GRID_SIZE, SPACE_GRID_SIZE), F32(Range::new(0., 1.))) * 0.1;
    u.add_assign(&u_rand);
    v.add_assign(&v_rand);
    (u, v)
}

// impl glium::texture::Texture2dDataSource<'_>` is not implemented for `ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 2]>>`
