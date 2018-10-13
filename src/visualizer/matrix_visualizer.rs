use failure;
use glium::{glutin, index, texture, Display, Program, Surface, VertexBuffer};
use ndarray::{ArrayBase, Dim, OwnedRepr};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use visualizer::WindowStatus;

/// 直交座標系(XY座標系)を用いてvisualizeする構造体
pub struct MatrixVisualizer {
    program: Program,
    events_loop: glutin::EventsLoop,
    vertex_buffer: VertexBuffer<Vertex>,
    indices: index::NoIndices,
    display: Display,
}

impl MatrixVisualizer {
    /// MatrixVisualizerインスタンスを生成する
    ///
    /// # Arguments
    /// * `title` - ウィンドウに表示するタイトル
    /// * `vertex_glsl_path` - バーテックスシェーダーのファイルを格納しているpath
    /// * `grafic_glsl_path` - グラフィックシェーダーのファイルを格納しているpath
    ///
    /// # Example
    /// ``````
    /// use my_alife::visualizer::matrix_visualizer::MatrixVisualizer;
    /// let matrix_visualize = MatrixVisualizer::new(
    ///   "Gray Scott",
    ///   "res/shaders/matrix_visualizer_vertex.glsl",
    ///   "res/shaders/matrix_visualizer_fragment.glsl",
    /// ).unwrap();
    ///
    ///
    pub fn new(
        title: &str,
        vertex_glsl_path: &str,
        faragment_glsl_path: &str,
    ) -> Result<MatrixVisualizer, failure::Error> {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_dimensions((600, 600).into())
            .with_title(title);
        let context = glutin::ContextBuilder::new();
        let display = Display::new(window, context, &events_loop).unwrap();
        let program = Program::from_source(
            &display,
            &Self::glsl(vertex_glsl_path)?,
            &Self::glsl(faragment_glsl_path)?,
            None,
        )?;

        let vertex_buffer = VertexBuffer::new(&display, &Self::shape()).unwrap();
        Ok(MatrixVisualizer {
            program: program,
            events_loop: events_loop,
            vertex_buffer: vertex_buffer,
            indices: index::NoIndices(index::PrimitiveType::TrianglesList),
            display: display,
        })
    }

    fn glsl(path: &str) -> Result<String, io::Error> {
        let mut contents = String::new();
        File::open(path)?.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn shape() -> Vec<Vertex> {
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
        vec![vertex1, vertex2, vertex3, vertex4, vertex5, vertex6]
    }

    /// メインループ
    ///
    /// # Arguments
    /// * `initail_state` - 初期状態
    /// * `unpdate_fn` - 描画する状態をどのように変更するかの関数
    ///
    /// # Example
    /// ```no_run
    /// extern crate ndarray;
    /// extern crate my_alife;
    ///
    /// use my_alife::visualizer::matrix_visualizer::{Matrix, MatrixVisualizer};
    /// use my_alife::algorithm::gray_scott::initial_matrix;
    /// use ndarray::Array2;
    ///
    /// let matrix = MatrixVisualizer::new(
    ///     "Gray Scott",
    ///     "res/shaders/matrix_visualizer_vertex.glsl",
    ///     "res/shaders/matrix_visualizer_fragment.glsl",
    /// );
    /// let state = (Array2::<f32>::ones((256, 256)), Array2::<f32>::ones((256, 256)));
    /// fn update_nothing(uv: &mut (Matrix<f32>, Matrix<f32>), f: f32, k: f32) {
    ///   &uv.0
    /// }
    ///
    /// matrix.unwrap().draw_loop(state, 0.04, 0.06, update_nothing);
    ///
    ///
    /// ```
    pub fn draw_loop<F>(
        mut self,
        mut state: (Matrix<f32>, Matrix<f32>),
        f: f32,
        k: f32,
        update_fn: F,
    ) -> Result<(), failure::Error>
    where
        F: Fn(&mut (Matrix<f32>, Matrix<f32>), f32, f32),
    {
        let mut window_status = WindowStatus::Open;
        loop {
            if window_status == WindowStatus::Close {
                break;
            }
            update_fn(&mut state, f, k);
            self.draw(&state.0)?;

            window_status = self.hadling_event();
        }
        Ok(())
    }

    /// 実際に描画を行う
    ///
    /// # Arguments
    /// * `matrix` - 描画される内容
    ///
    pub fn draw(&self, matrix: &Matrix<f32>) -> Result<(), failure::Error> {
        let image = make_texture_image(matrix);
        let texture = texture::Texture2d::new(&self.display, image);
        let mut target = self.display.draw();
        target.clear_color(1.0, 0.0, 0.0, 1.0);
        target.draw(
            &self.vertex_buffer,
            &self.indices,
            &self.program,
            &uniform! {u_texture: texture?.sampled()},
            &Default::default(),
        )?;
        target.finish()?;
        Ok(())
    }

    /// event handler
    pub fn hadling_event(&mut self) -> WindowStatus {
        let mut status = WindowStatus::Open;
        self.events_loop.poll_events(|event| {
            // matchさせたいパターンが1つしかない場合、if let 形式で書ける
            // matchでやると
            // match event {
            //   glutin::Event::WindowEvent { event, .. } => { do_something() },
            //   _ => { // do nothing }}
            // }
            // みたいにcatch節的なものが必要になる(rustのパターンマッチは取り得る全パターンを明示的に書かせるため)
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::CloseRequested => status = WindowStatus::Close,
                    glutin::WindowEvent::KeyboardInput {
                        device_id: _,
                        input: keyboard_input,
                    } => match keyboard_input {
                        glutin::KeyboardInput { // 構造体の各fieldをdestructuringできる
                            virtual_keycode, // virtual_keycode: virtual_keycode を省略形
                            modifiers, // modifiers: my_modifiers の様に省略しないで別名をつけても良い
                            .. // 使わないfieldのscancode: _, state: _, を省略できる
                        } => match (virtual_keycode, modifiers) { // 複数のパターンマッチにはタプルを使う
                            #[cfg(target_os = "linux")] // conditional compile https://doc.rust-lang.org/reference/attributes.html#conditional-compilation
                            (Some(glutin::VirtualKeyCode::W), glutin::ModifiersState { ctrl, .. }) => {
                              if ctrl { status = WindowStatus::Close }
                            },
                            #[cfg(target_os = "macos")]
                            (Some(glutin::VirtualKeyCode::W), glutin::ModifiersState { logo, .. }) => {
                              if logo { status = WindowStatus::Close }
                            },
                            (_, _) => {}
                        },
                    },
                    _ => {}
                }
            };
        });
        return status;
    }
}

/// 直交座標系(XY座標系)においてどの座標にどんな色(グレースケール)を表示するかを表現する。  
/// 実体は2次元配列
pub type Matrix<T> = ArrayBase<OwnedRepr<T>, Dim<[usize; 2]>>;

#[derive(Copy, Clone)]
struct Vertex {
    a_position: [f32; 2],
    a_texcoord: [f32; 2],
}
implement_vertex!(Vertex, a_position, a_texcoord);

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
    texture::RawImage2d::from_raw_rgba(texture_data, (u.shape()[0] as u32, u.shape()[1] as u32))
}
