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
use glium::texture::{Texture2dDataSource, PixelValue};

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
}

fn draw_triangle(u: ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 2]>>) {
    use glium::{glutin, Surface};
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions((600, 600).into())
        .with_title("Hello world");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        a_position: [f32; 2],
        a_texcoord: [f32; 2],
    }
    implement_vertex!(Vertex, a_position, a_texcoord);

    // #[derive(Copy, Clone)]
    // struct VTexcoord {
    //     v_texcoord: [f32; 2],
    // }
    // implement_vertex!(VTexcoord, v_texcoord);
    let vertex1 = Vertex {
        a_position: [-1.0, -1.0],
        a_texcoord: [0.0, 0.0],
    };
    let vertex2 = Vertex {
        a_position: [1.0, 1.0],
        a_texcoord: [1.0, 0.0],
    };
    let vertex3 = Vertex {
        a_position: [1.0, -1.0],
        a_texcoord: [1.0, 0.0],
    };
    let vertex4 = Vertex {
        a_position: [-1.0, -1.0],
        a_texcoord: [0.0, 0.0],
    };
    let vertex5 = Vertex {
        a_position: [1.0, 1.0],
        a_texcoord: [1.0, 0.0],
    };
    let vertex6 = Vertex {
        a_position: [-1.0, 1.0],
        a_texcoord: [0.0, 1.0],
    };
    let vertex11 = Vertex {
        a_position: [-1.0, -1.0],
        a_texcoord: [0.0, 1.0],
    };
    let vertex12 = Vertex {
        a_position: [1.0, -1.0],
        a_texcoord: [1.0, 1.0],
    };
    let vertex13 = Vertex {
        a_position: [1.0, 1.0],
        a_texcoord: [1.0, 0.0],
    };
    let vertex14 = Vertex {
        a_position: [-1.0, -1.0],
        a_texcoord: [0.0, 1.0],
    };
    let vertex15 = Vertex {
        a_position: [-1.0, 1.0],
        a_texcoord: [0.0, 0.0],
    };
    let vertex16 = Vertex {
        a_position: [1.0, 1.0],
        a_texcoord: [1.0, 0.0],
    };
    let shape = vec![vertex11, vertex12, vertex13, vertex14, vertex15, vertex16];
    // let shape = vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6];
    // let shape = vec![vertex1, vertex2, vertex3];
    // let shape = vec![vertex4, vertex5, vertex6];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    // let a_texcoord1 = ATexcoord { a_texcoord: [0.0, 1.0] };
    // let a_texcoord2 = ATexcoord { a_texcoord: [0.0, 0.0] };
    // let a_texcoord3 = ATexcoord { a_texcoord: [1.0, 1.0] };
    // let a_texcoord4 = ATexcoord { a_texcoord: [1.0, 0.0] };
    // let a_texcoord5 = ATexcoord { a_texcoord: [1.0, 1.0] };
    // let a_texcoord6 = ATexcoord { a_texcoord: [0.0, 0.0] };
    // let a_shape = vec![
    //     a_texcoord1,
    //     a_texcoord2,
    //     a_texcoord3,
    //     a_texcoord4,
    //     a_texcoord5,
    //     a_texcoord6,
    // ];
    // let a_texcoord_buffer = glium::VertexBuffer::new(&display, &a_shape).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let v = include_str!("../res/shaders/matrix_visualizer_vertex.glsl");
    let f = include_str!("../res/shaders/matrix_visualizer_fragment.glsl");
    let program0 = glium::Program::from_source(
        &display,
        include_str!("../res/shaders/matrix_visualizer_vertex.glsl"),
        include_str!("../res/shaders/matrix_visualizer_fragment.glsl"),
        None,
    );
    println!("{:?}", program0);
    let program = program0.unwrap();

    let vector = u.iter().map(|d| d).collect::<Vec<&f32>>();
    // println!("{:?}", vector);
    // let mut texture_data: Vec<Vec<(u32, u32, u32, u32)>> = Vec::new();
    let mut texture_data = Vec::new();
    for i in 0..256 {
        // let mut inner_vec: Vec<(u32, u32, u32, u32)> = Vec::new();
        // let mut inner_vec = Vec::new();
        for j in 0..256 {
            let mut v = vector[i * 256 + j].clone();
            if v < 0.0 {
                v = 0.0;
            } else if v > 1.0 {
                v = 1.0;
            } else {
            }
            v = v * 255.0;
            let uv = v as u8;

            // inner_vec.push(vector[i + j].clone())
            // inner_vec.push(vec![v, v, v, v]);
            // inner_vec.push((uv, uv, uv, 1));
            // inner_vec.push((0, 255, 255 as u32));
            // inner_vec.push((255, 255, 255 as u32));
            // inner_vec.push(255 as u32);
            // texture_data.push(124 as u8);
            // texture_data.push(125 as u8);
            // texture_data.push(255 as u8);
            // texture_data.push(255 as u8);
            // texture_data.push(255 as u8);
            texture_data.push(uv);
            texture_data.push(uv);
            texture_data.push(uv);
            texture_data.push(uv);
        }
        // texture_data.push(inner_vec);
    }
    // let texture_data = vec![vec![0.0]];

    // let texture_data = [[1.0; 256]; 256];
    // let texture_data = u;
    // println!("{:?}", texture_data);
    // let texture = glium::texture::Texture2d::new(&display, texture_data).unwrap();
    println!("raw: {}", texture_data.len());
    let image = glium::texture::RawImage2d::from_raw_rgba(texture_data, (256, 256));
    // let image = glium::texture::RawImage2d::from_raw_rgba(texture_data, (256, 256));
    println!(
        "widhth{:?}, height: {:?}, format: {:?}",
        image.width,
        image.height,
        image.format
    );
    // println!("{:?}", image.data);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();
    // let texture = glium::texture::CompressedTexture2d::new(&display, texture_data).unwrap();
    println!("{:?}", texture);
    println!(
        "width: {}, height: {:?}",
        texture.get_width(),
        texture.get_height()
    );

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(1.0, 0.0, 0.0, 1.0);
        target
            .draw(
                // (&vertex_buffer, &a_texcoord_buffer, &v_texcoord_buffer),
                &vertex_buffer,
                &indices,
                &program,
                // &glium::uniforms::EmptyUniforms,
                // &uniform! {matrix: [0.0, 0.0, 0.0, 0.0]},
                &uniform! {
                u_texture: texture.sampled()
                               },
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                }
            }
            _ => (),
        });
    }
}

fn make_matrix()
    -> (ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 2]>>,
        ndarray::ArrayBase<ndarray::OwnedRepr<f32>, ndarray::Dim<[usize; 2]>>)
{
    // initialize
    let mut u = Array2::<f32>::ones((256, 256));
    let mut v = Array2::<f32>::zeros((256, 256));
    const SQUARE_SIZE: usize = 20;
    u.slice_mut(s![
        // 0..256,
        // 0..256,
        SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..
            SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
        // 0..256,
        SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..
            SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
    ]).fill(0.5);
    // println!("{:?}",
    // u.slice(s![
    //     SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..
    //         SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
    //     SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..
    //         SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
    // ]));
    v.slice_mut(s![
        SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..
            SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
        SPACE_GRID_SIZE / 2 - SQUARE_SIZE / 2..
            SPACE_GRID_SIZE / 2 + SQUARE_SIZE / 2,
    ]).fill(0.25);
    let u_rand = Array::random((SPACE_GRID_SIZE, SPACE_GRID_SIZE), F32(Range::new(0., 1.))) * 0.1;
    let v_rand = Array::random((SPACE_GRID_SIZE, SPACE_GRID_SIZE), F32(Range::new(0., 1.))) * 0.1;
    u.add_assign(&u_rand);
    v.add_assign(&v_rand);

    // println!("{:?}", u);
    (u, v)
}

// impl<'a, P: PixelValue + Clone> Texture2dDataSource<'a> for Vec<Vec<[u32; 4]>> {
//     type Data = P;
//
//     fn into_raw(self) -> RawImage2d<'a, P> {
//         let width = self.iter().next().map(|e| e.len()).unwrap_or(0) as u32;
//         let height = self.len() as u32;
//
//         RawImage2d {
//             data: Cow::Owned(self.into_iter().flat_map(|e| e.into_iter()).collect()),
//             width: width,
//             height: height,
//             format: <P as PixelValue>::get_format(),
//         }
//     }
// }
