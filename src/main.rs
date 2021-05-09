mod camera;
mod cube;
mod vectormath;
mod vertex;
mod meshgen;

use camera::*;
use vertex::{Normal, Vertex};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern crate glium;
use glium::{Display, Program, ProgramCreationError, Surface, glutin::{self, event::VirtualKeyCode}};

extern crate image;

const SCENE_LIGHT: [f32; 3] = [-1.0, 0.701, -1.0];

fn load_shader(display: &Display, vertex_path: &str, fragment_path: &str) -> Result<Program, ProgramCreationError> {
    let mut tri_vertex_shader_file = match File::open(Path::new(vertex_path)) {
        Err(why) => panic!("Could not open file: {}", why),
        Ok(file) => file,
    };
    let mut tri_vertex_shader_str = String::new();
    match tri_vertex_shader_file.read_to_string(&mut tri_vertex_shader_str) {
        Err(why) => panic!("Could not read file: {}", why),
        Ok(_) => (),
    }
    let mut tri_fragment_shader_file = match File::open(Path::new(fragment_path)) {
        Err(why) => panic!("Could not open file: {}", why),
        Ok(file) => file,
    };
    let mut tri_fragment_shader_str = String::new();
    match tri_fragment_shader_file.read_to_string(&mut tri_fragment_shader_str) {
        Err(why) => panic!("Could not read file: {}", why),
        Ok(_) => (),
    }

    let shader = glium::Program::from_source(
        display,
        tri_vertex_shader_str.as_str(),
        tri_fragment_shader_str.as_str(),
        None,
    );
    shader
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
        point_size: Some(1.0),
        line_width: Some(1.0),
        ..Default::default()
    };
    let cube_shader = load_shader(&display, "shaders/cube_vertex.glsl", "shaders/cube_fragment.glsl").unwrap();

    //let mut cube1 = cube::Cube::new([-1.0, 1.0, 5.0], &display, None, [0.6, 0.2, 0.2]);
    //let mut cube2 = cube::Cube::new([1.0, 0.0, 5.0], &display, None, [0.22, 0.6, 0.1]);
    let mut camera = Camera::new(&[0.0, 0.0, 0.0], &[0.0, 0.0, 5.0]);

    /*let mut cubes: Vec<cube::Cube> = Vec::with_capacity(5 * 5);
    for x in 0..5 {
        for z in 0..5 {
            cubes.push(cube::Cube::new([x as f32, 1.0, z as f32], &display, None, [0.6, 0.2, 0.2]));
        }
    }*/

    //TODO: offset each face added to the mesh by its position
    //Then it should be ready to draw
    let chunk: [[u8; 16]; 16] = [[0u8; 16]; 16];
    let mesh: (Vec<Vertex>, Vec<Normal>) = meshgen::gen_chunk_mesh(&chunk);

    event_loop.run(move |ev, _, control_flow| {
        let mut target = display.draw();

        target.clear_color_and_depth((0.1, 0.15, 0.9, 1.0), 1.0);

        /*for c in &cubes {
            c.draw(&mut target, &params, &camera, &cube_shader);
        }*/

        target.finish().unwrap();

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::DeviceEvent { event, .. } => match event {
                glutin::event::DeviceEvent::Key(k) => match k.virtual_keycode {
                    _ => {
                        if let Some(code) = k.virtual_keycode {
                            println!("KeyCode: {:?}", code);
                            match code {
                                VirtualKeyCode::W => camera.move_direction(&[0.0, 0.0, 0.5]),
                                VirtualKeyCode::S => camera.move_direction(&[0.0, 0.0, -0.5]),
                                VirtualKeyCode::A => camera.move_direction(&[-0.5, 0.0, 0.0]),
                                VirtualKeyCode::D => camera.move_direction(&[0.5, 0.0, 0.0]),
                                VirtualKeyCode::Space => camera.move_direction(&[0.0, 0.5, 0.0]),
                                VirtualKeyCode::LShift => camera.move_direction(&[0.0, -0.5, 0.0]),

                                //VirtualKeyCode::C => t += 0.05,

                                VirtualKeyCode::Up => camera.translate(&[0.0, 0.01, 0.0]),
                                VirtualKeyCode::Down => camera.translate(&[0.0, -0.01, 0.0]),

                                _ => (),
                            }
                        }
                    }
                },
                glutin::event::DeviceEvent::Motion { axis, value } => match axis {
                    0 => camera.rotate_on_y_axis(value as f32 * 0.001),
                    1 => camera.rotate_on_x_axis(value as f32 * 0.001),
                    _ => {}
                },
                _ => {}
            },
            _ => (),
        }
    });
}
