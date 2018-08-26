extern crate gl;
extern crate glutin;
#[macro_use(s)]
extern crate ndarray;
extern crate ndarray_rand;
extern crate rand;

use glutin::dpi::*;
use glutin::GlContext;
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
    println!("{:?}", v);

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("chap2")
        .with_dimensions(LogicalSize::new(1024.0, 768.0));
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    // let mut running = true;
    let mut running = false;
    while running {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => running = false,
                glutin::WindowEvent::Resized(logical_size) => {}
                _ => {}
            },
            _ => {}
        });
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        gl_window.swap_buffers().unwrap();
    }
}
