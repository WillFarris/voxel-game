mod camera;
mod cube;
mod vectormath;
mod vertex;
mod meshgen;
mod block;

use block::{Block, TextureType};
use camera::*;
use vertex::{Vertex, Vertex2D};

use std::fs::File;
use std::io::{prelude::*, Cursor};
use std::path::Path;
use std::include_bytes;

extern crate glium;
use glium::{Display, Program, ProgramCreationError, Surface, VertexBuffer, glutin::{self, event::{ElementState, VirtualKeyCode}}, uniform, uniforms::{MagnifySamplerFilter, MinifySamplerFilter}};

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
        ..Default::default()
    };
    let cube_shader = load_shader(&display, "shaders/cube_vertex.glsl", "shaders/cube_fragment.glsl").unwrap();
    let ui_shader = load_shader(&display, "shaders/ui_vertex.glsl", "shaders/ui_fragment.glsl").unwrap();

    let ui_crosshair_mesh = [
        Vertex2D { position: (-0.01, 0.01), tex_coords: (0.0, 1.0) },
        Vertex2D { position: (-0.01, -0.01), tex_coords: (0.0, 0.0) },
        Vertex2D { position: (0.01, 0.01), tex_coords: (1.0, 1.0) },

        Vertex2D { position: (0.01, 0.01), tex_coords: (1.0, 1.0) },
        Vertex2D { position: (0.01, -0.01), tex_coords: (1.0, 0.0) },
        Vertex2D { position: (-0.01, -0.01), tex_coords: (0.0, 0.0) },
    ];
    let ui_crosshair_buffer = glium::VertexBuffer::new(&display, &ui_crosshair_mesh).unwrap();

    let mut cube1 = cube::Cube::new([-1.0, 5.0, 5.0], &display, None, [0.9, 0.2, 0.2]);
    //let mut cube2 = cube::Cube::new([1.0, 0.0, 5.0], &display, None, [0.22, 0.6, 0.1]);
    let mut camera = Camera::new(&[8.0, 6.0, 0.0], &[0.0, 0.0, 0.10]);

    let terrain_image = image::load(Cursor::new(&include_bytes!("../terrain.png")),
                        image::ImageFormat::Png).unwrap().to_rgba8();
    let terrain_image_dimensions = terrain_image.dimensions();
    let terrain_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&terrain_image.into_raw(), terrain_image_dimensions);
    let terrain_texture = glium::texture::SrgbTexture2d::new(&display, terrain_image).unwrap();
    let behavior = glium::uniforms::SamplerBehavior {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
    };

    let crosshair_image = image::load(Cursor::new(&include_bytes!("../crosshair.png")),
                        image::ImageFormat::Png).unwrap().to_rgba8();
    let crosshair_image_dimensions = crosshair_image.dimensions();
    let crosshair_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&crosshair_image.into_raw(), crosshair_image_dimensions);
    let crosshair_texture = glium::texture::SrgbTexture2d::new(&display, crosshair_image).unwrap();
    let behavior = glium::uniforms::SamplerBehavior {
        minify_filter: MinifySamplerFilter::Nearest,
        magnify_filter: MagnifySamplerFilter::Nearest,
        ..Default::default()
    };

    let mut chunk: [[[Block; 16]; 16]; 16] = [[[Block::default(); 16]; 16]; 16];
    for x in 0..16 {
        for y in 0..4 {
            for z in 0..16 {
                chunk[x][y][z] = Block::new(1, Some(TextureType::Single(2,12)));
            }
        }
    }
    let mut mesh: Vec<Vertex> = meshgen::gen_chunk_mesh(&chunk);
    let mut mesh_vertex_buffer = VertexBuffer::new(&display, mesh.as_slice()).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        /*let next_frame_time = std::time::Instant::now() +
        std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);*/
        let mut target = display.draw();

        target.clear_color_and_depth((0.1, 0.15, 0.9, 1.0), 1.0);


        cube1.draw(&mut target, &params, &camera, &cube_shader);
        
        
        target
            .draw(
                &mesh_vertex_buffer,
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
                    tex: glium::uniforms::Sampler(&terrain_texture, behavior),
                },
                &params,
            )
            .unwrap();

            let dimensions = target.get_dimensions();
            target
            .draw(
                &ui_crosshair_buffer,
                glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &ui_shader,
                &uniform! {
                    u_dimensions: [dimensions.0 as f32, dimensions.1 as f32],
                    u_texture: glium::uniforms::Sampler(&crosshair_texture, behavior),
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

                                VirtualKeyCode::C => {},
                                VirtualKeyCode::Escape => *control_flow = glutin::event_loop::ControlFlow::Exit,

                                _ => (),
                            }
                        }
                    }
                },
                glutin::event::DeviceEvent::Button { button, state } => match button {
                    1 => {
                        if state == ElementState::Pressed {
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
                                                println!("Chunk intersection with block at ({}, {}, {}): {}", x, y, z, chunk[x][y][z].id);
                                                if chunk[x][y][z].id != 0 {
                                                    hit = true;
                                                    //cube1.translate(&[x as f32, y as f32, z as f32]);
                                                    chunk[x][y][z] = Block::default();
                                                    mesh = meshgen::gen_chunk_mesh(&chunk);
                                                    mesh_vertex_buffer = VertexBuffer::new(&display, mesh.as_slice()).unwrap();
                                                    steps = hit_radius;
                                                    break;
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
                        }
                    },
                    _ => {}
                }
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
