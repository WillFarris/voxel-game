mod cube;
mod vertex;
mod teapot;
mod camera;


use teapot::Normal;
use teapot::Vertex;

use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::Path;

extern crate glium;
use glium::{DrawParameters, IndexBuffer, Program, Surface, VertexBuffer, glutin::{self, event::VirtualKeyCode}, uniform};

extern crate image;

const SCENE_LIGHT: [f32; 3] = [3.0, 3.0, -2.0];


fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

struct Teapot {
    transform_matrix: [[f32;4];4],
    shader: glium::Program,
    v_positions: VertexBuffer<Vertex>,
    v_normals: VertexBuffer<Normal>,
    v_indices: IndexBuffer<u16>,
}

impl Teapot {
    fn new(position: [f32; 3], display: &glium::Display, shader: Program) -> Self {
        Self {
            transform_matrix: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [position[0], position[1], position[2] + 5.0, 1.0],
            ],
            shader,
            v_positions: glium::VertexBuffer::new(display, &teapot::VERTICES).unwrap(),
            v_normals: glium::VertexBuffer::new(display, &teapot::NORMALS).unwrap(),
            v_indices: glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap(),
        }
    }

    fn translate(&mut self, translation: [f32; 3]) {
        self.transform_matrix[3][0] += translation[0];
        self.transform_matrix[3][1] += translation[1];
        self.transform_matrix[3][2] += translation[2];
    }

    fn draw(&self, target: &mut glium::Frame, params: &DrawParameters) {
        target.draw((&self.v_positions, &self.v_normals), &self.v_indices,&self.shader, &uniform! { matrix: self.transform_matrix, perspective: camera::perspective_matrix(&target), light: SCENE_LIGHT}, &params).unwrap();
    }
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    /* Read the vertex and fragment shaders from disk and create a program */
    let mut tri_vertex_shader_file = match File::open(Path::new("shaders/tri_vertex.glsl")) {
        Err(why) => panic!("Could not open file: {}", why),
        Ok(file) => file,
    };
    let mut tri_vertex_shader_str = String::new();
    match tri_vertex_shader_file.read_to_string(&mut tri_vertex_shader_str) {
        Err(why) => panic!("Could not read file: {}", why),
        Ok(_) => (),
    }
    let mut tri_fragment_shader_file = match File::open(Path::new("shaders/tri_fragment.glsl")) {
        Err(why) => panic!("Could not open file: {}", why),
        Ok(file) => file,
    };
    let mut tri_fragment_shader_str = String::new();
    match tri_fragment_shader_file.read_to_string(&mut tri_fragment_shader_str) {
        Err(why) => panic!("Could not read file: {}", why),
        Ok(_) => (),
    }
    // Ownership of the shader will be given to pot    
    let teapot_shader = glium::Program::from_source(
        &display,
        tri_vertex_shader_str.as_str(),
        tri_fragment_shader_str.as_str(),
        None,
    )
    .unwrap();

    let mut pot = Teapot::new([1.0, 1.0, 1.0], &display, teapot_shader);

    let mut t: f32 = 0.0;
    event_loop.run(move |ev, _, control_flow| {
        t += 0.001;
        let mut target = display.draw();

        target.clear_color_and_depth((0.1, 0.15, 0.9, 1.0), 1.0);

        pot.draw(&mut target, &params);

        target.finish().unwrap();

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::DeviceEvent {event, ..} => match event {
                glutin::event::DeviceEvent::Key(k) => match k.virtual_keycode {
                    _ => {
                        if let Some(code) = k.virtual_keycode {
                            match code {
                                VirtualKeyCode::W =>      pot.translate([0.0,   0.0,  0.2]),
                                VirtualKeyCode::S =>      pot.translate([0.0,   0.0, -0.2]),
                                VirtualKeyCode::A =>      pot.translate([-0.2,  0.0,  0.0]),
                                VirtualKeyCode::D =>      pot.translate([0.2,   0.0,  0.0]),
                                VirtualKeyCode::Space =>  pot.translate([0.0,   0.2,  0.0]),
                                VirtualKeyCode::LShift => pot.translate([0.0,  -0.2,  0.0]),
                                _ => ()
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => (),
        }
    });
}
