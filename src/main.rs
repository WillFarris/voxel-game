mod camera;
mod cube;
mod vectormath;
mod vertex;
mod meshgen;

use camera::*;
use cube::Cube;
use meshgen::gen_chunk_mesh;
use vertex::{Normal, Vertex};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::cmp::{min, max};

extern crate glium;
use glium::{Display, Program, ProgramCreationError, Surface, VertexBuffer, glutin::{self, event::VirtualKeyCode}, uniform};

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
    let wb = glutin::window::WindowBuilder::new().with_title("A game").with_maximized(false);
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        point_size: Some(1.0),
        line_width: Some(1.0),
        ..Default::default()
    };
    let cube_shader = load_shader(&display, "shaders/cube_vertex.glsl", "shaders/cube_fragment.glsl").unwrap();

    let mut cube1 = cube::Cube::new([-1.0, 5.0, 5.0], &display, None, [0.9, 0.2, 0.2]);
    let mut cubes: Vec<Cube> = Vec::new();
    //let mut cube2 = cube::Cube::new([1.0, 0.0, 5.0], &display, None, [0.22, 0.6, 0.1]);
    let mut camera = Camera::new(&[8.0, 6.0, 0.0], &[0.0, 0.0, 0.10]);


    let mut chunk: [[[u8; 16]; 16]; 16] = [[[0u8; 16]; 16]; 16];
    /*for x in 0..16 {
        for y in 0..4 {
            for z in 0..16 {
                chunk[x][y][z] = 1;
            }
        }
    }*/

    chunk[0][1][0] = 1;
    let mesh: (Vec<Vertex>, Vec<Normal>) = meshgen::gen_chunk_mesh(&chunk);
    let mesh_vertex_buffer = VertexBuffer::new(&display, mesh.0.as_slice()).unwrap();
    let mesh_normal_buffer = VertexBuffer::new(&display, mesh.1.as_slice()).unwrap();


    
    //mesh = meshgen::gen_chunk_mesh(&chunk);
    //mesh_vertex_buffer = VertexBuffer::new(&display, mesh.0.as_slice()).unwrap();
    //mesh_normal_buffer = VertexBuffer::new(&display, mesh.1.as_slice()).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        let next_frame_time = std::time::Instant::now() +
        std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        let mut target = display.draw();

        target.clear_color_and_depth((0.1, 0.15, 0.9, 1.0), 1.0);

        for c in &cubes {
            c.draw(&mut target, &params, &camera, &cube_shader);
        }

        cube1.draw(&mut target, &params, &camera, &cube_shader);
        
        
        target
            .draw(
                (&mesh_vertex_buffer, &mesh_normal_buffer),
                glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &cube_shader,
                &uniform! {
                    model_matrix: vectormath::IDENTITY_MAT4,
                    view_matrix: camera.view_matrix(),
                    perspective_matrix: crate::camera::perspective_matrix(&target),
                    light: crate::SCENE_LIGHT,
                    u_color: [0.3, 0.8, 0.3f32],
                    u_position: camera.position,
                    u_direction: camera.forward,
                },
                &params,
            )
            .unwrap();

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
                            //println!("KeyCode: {:?}", code);
                            match code {
                                VirtualKeyCode::W => camera.move_direction(&[0.0, 0.0, 0.5]),
                                VirtualKeyCode::S => camera.move_direction(&[0.0, 0.0, -0.5]),
                                VirtualKeyCode::A => camera.move_direction(&[-0.5, 0.0, 0.0]),
                                VirtualKeyCode::D => camera.move_direction(&[0.5, 0.0, 0.0]),
                                VirtualKeyCode::Space => camera.move_direction(&[0.0, 0.5, 0.0]),
                                VirtualKeyCode::LShift => camera.move_direction(&[0.0, -0.5, 0.0]),

                                VirtualKeyCode::Up => camera.rotate_on_x_axis(-0.05),
                                VirtualKeyCode::Down => camera.rotate_on_x_axis(0.05),
                                VirtualKeyCode::Left => camera.rotate_on_y_axis(-0.05),
                                VirtualKeyCode::Right => camera.rotate_on_y_axis(0.05),

                                VirtualKeyCode::C => {
                                    let mut ray = (
                                        camera.position[0],
                                        camera.position[1],
                                        camera.position[2],
                                    );
                                    let mut hit = false;
                                    let hit_radius = 20;
                                    let mut steps = 0;
                                    while !hit && steps < hit_radius {
                                        for x in 0..15 {
                                            for y in 0..15 {
                                                for z in 0..15 {
                                                    if ray.0 > (x as f32)-0.5 && ray.0 < (x as f32)+0.5
                                                    && ray.1 > (y as f32)-0.5 && ray.1 < (y as f32)+0.5
                                                    && ray.2 > (z as f32)-0.5 && ray.2 < (z as f32)+0.5 {
                                                        println!("Chunk intersection with block at ({}, {}, {}): {}", x, y, z, chunk[x][y][z]);
                                                        if chunk[x][y][z] != 0 {
                                                            hit = true;
                                                            cube1.translate(&[x as f32, y as f32, z as f32]);
                                                            //chunk[x][y][z] = 0;
                                                            //mesh = meshgen::gen_chunk_mesh(&chunk);
                                                            //mesh_vertex_buffer = VertexBuffer::new(&display, mesh.0.as_slice()).unwrap();
                                                            //mesh_normal_buffer = VertexBuffer::new(&display, mesh.1.as_slice()).unwrap();
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        ray.0 += 0.5 * camera.forward[0];
                                        ray.1 += 0.5 * camera.forward[1];
                                        ray.2 += 0.5 * camera.forward[2];
                                        steps += 1;
                                    }
                                },
                                VirtualKeyCode::Escape => *control_flow = glutin::event_loop::ControlFlow::Exit,

                                _ => (),
                            }
                        }
                    }
                },
                /*glutin::event::DeviceEvent::Button { button, state } => match button {
                    1 => 
                    }
                    _ => {}
                }*/
                /*glutin::event::DeviceEvent::Motion { axis, value } => match axis {
                    0 => camera.rotate_on_y_axis(value as f32 * 0.001),
                    1 => camera.rotate_on_x_axis(value as f32 * 0.001),
                    _ => {}
                },*/
                _ => {}
            },
            _ => (),
        }
    });
}
