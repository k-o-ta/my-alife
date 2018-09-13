use failure;
use glium::{glutin, index, program, texture, Display, Program, Surface, VertexBuffer};
use ndarray::{ArrayBase, Dim, OwnedRepr};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

struct MatrixVisualizer {
    program: Program,
    events_loop: glutin::EventsLoop,
    vertex_buffer: VertexBuffer<Vertex>,
    indices: index::NoIndices,
    display: Display,
}

impl MatrixVisualizer {
    fn glsl(path: String) -> Result<String, io::Error> {
        let mut contents = String::new();
        File::open(path)?.read_to_string(&mut contents)?;
        Ok(contents)
    }
    pub fn new(
        vertex_glsl_path: String,
        faragment_glsl_path: String,
        // ) -> Result<MatrixVisualizer, program::ProgramCreationError> {
    ) -> Result<MatrixVisualizer, failure::Error> {
        let mut events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_dimensions((600, 600).into())
            .with_title("Gray Scott");
        let context = glutin::ContextBuilder::new();
        let display = Display::new(window, context, &events_loop).unwrap();
        let program = Program::from_source(
            &display,
            &Self::glsl(vertex_glsl_path)?,
            &Self::glsl(faragment_glsl_path)?,
            // include_str!("../../res/shaders/matrix_visualizer_vertex.glsl"),
            // include_str!("../../res/shaders/matrix_visualizer_fragment.glsl"),
            None,
        )?;
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

        let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
        Ok(MatrixVisualizer {
            program: program,
            events_loop: events_loop,
            vertex_buffer: vertex_buffer,
            indices: index::NoIndices(index::PrimitiveType::TrianglesList),
            display: display,
        })
    }
    pub fn draw<T, F>(mut self, mut initialState: T, mut closure: F)
    where
        F: FnMut(&mut T) -> &Matrix<f32>,
    {
        let mut closed = false;
        loop {
            if closed {
                break;
            }
            let u = closure(&mut initialState);
            let image = make_texture_image(u);
            let texture = texture::Texture2d::new(&self.display, image).unwrap();
            let mut target = self.display.draw();
            target.clear_color(1.0, 0.0, 0.0, 1.0);
            target.draw(
                &self.vertex_buffer,
                &self.indices,
                &self.program,
                &uniform! {u_texture: texture.sampled()},
                &Default::default(),
            );
            target.finish().unwrap();

            self.events_loop.poll_events(|event| {
                if let glutin::Event::WindowEvent { event, .. } = event {
                    if let glutin::WindowEvent::CloseRequested = event {
                        closed = true
                    }
                }
            });
        }
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    a_position: [f32; 2],
    a_texcoord: [f32; 2],
}
implement_vertex!(Vertex, a_position, a_texcoord);

pub type Matrix<T> = ArrayBase<OwnedRepr<T>, Dim<[usize; 2]>>;

pub fn draw<T, F>(mut initialState: T, mut closure: F)
where
    F: FnMut(&mut T) -> &Matrix<f32>,
{
    use glium::{glutin, Surface};
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions((600, 600).into())
        .with_title("Gray Scott");
    let context = glutin::ContextBuilder::new();
    let display = Display::new(window, context, &events_loop).unwrap();

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

    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();

    let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

    let program0 = Program::from_source(
        &display,
        include_str!("../../res/shaders/matrix_visualizer_vertex.glsl"),
        include_str!("../../res/shaders/matrix_visualizer_fragment.glsl"),
        None,
    );
    let program = program0.unwrap();

    let mut closed = false;
    while !closed {
        let u = closure(&mut initialState);

        let image = make_texture_image(u);
        let texture = texture::Texture2d::new(&display, image).unwrap();
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

fn make_texture_image<'a>(u: &Matrix<f32>) -> texture::RawImage2d<'a, u8> {
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
    texture::RawImage2d::from_raw_rgba(texture_data, (256, 256))
}
