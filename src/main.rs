mod camera;
mod cube;
mod vectormath;
mod vertex;
mod meshgen;
mod mesh;
mod block;
mod util;
mod shader;
mod macros;

use block::{Block, TextureType};
use camera::*;
use cgmath::{Matrix4, Vector2, Vector3};
use glfw::Context;
use mesh::{Mesh, Texture, texture_from_file};
use meshgen::gen_chunk_mesh;
use std::ffi::{CString, CStr};
use vertex::Vertex;

use std::sync::mpsc::Receiver;

extern crate image;
extern crate glfw;
extern crate gl;

const SCENE_LIGHT: [f32; 3] = [-1.0, 0.701, -1.0];

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 1024;

fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(WIDTH, HEIGHT, "", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");


    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos(WIDTH as f64/2.0, HEIGHT as f64/2.0);
    //window.set_cursor_mode(glfw::CursorMode::Hidden);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let shader = shader::Shader::new("shaders/cube_vertex.glsl", "shaders/cube_fragment.glsl");


    //let mut cube1 = cube::Cube::new([-1.0, 5.0, 5.0], [0.9, 0.2, 0.2]);
    let mut chunk: [[[Block; 16]; 16]; 16] = [[[Block::default(); 16]; 16]; 16];
    for x in 0..16 {
        for y in 0..4 {
            for z in 0..16 {
                chunk[x][y][z] = block::MOSSY_COBBLESTONE;
            }
        }
    }

    let texture_id = texture_from_file("terrain.png", ".");
    let mesh_texture = Texture {id: texture_id};
    let mut mesh_vertices = gen_chunk_mesh(&chunk);
    let mut mesh = mesh::Mesh::new(mesh_vertices, &mesh_texture, &shader);

    let mut camera = Camera::new(Vector3::new(-8.0, 11.0, -9.0), Vector3::new(0.64, 0.545, 0.52));
    camera.set_move_speed(0.5);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
    
    while !window.should_close() {
        unsafe {
            
            gl::ClearColor(0.1, 0.2, 0.5, 1.0);
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            
            shader.use_program();

            let projection: Matrix4<f32> = perspective_matrix();//cgmath::perspective(cgmath::Deg(90.0), WIDTH as f32 / HEIGHT as f32, 0.1, 100.0);
            let view = camera.view_matrix();
            let model = Matrix4::from_scale(1.0);

            shader.set_mat4(c_str!("perspective_matrix"), &projection);
            shader.set_mat4(c_str!("view_matrix"), &view);
            shader.set_mat4(c_str!("model_matrix"), &model);
            mesh.draw();

        }
        
        
        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::CursorPos(x, y) => {
                    let delta = (x-WIDTH as f64/2.0, y-HEIGHT as f64/2.0);
                    window.set_cursor_pos(WIDTH as f64/2.0, HEIGHT as f64/2.0);
                    camera.rotate_on_x_axis(0.001 * delta.1 as f32);
                    camera.rotate_on_y_axis(0.001 * delta.0 as f32);
                },
                glfw::WindowEvent::MouseButton(button, action, modifiers) => {
                    match button {
                        glfw::MouseButton::Button1 => {
                            if action == glfw::Action::Press {
                                let mut ray = (
                                    camera.position[0],
                                    camera.position[1],
                                    camera.position[2],
                                );
                                let mut hit = false;
                                let hit_radius = 20;
                                let mut steps = 0;
                                while !hit && steps < hit_radius {
                                    for x in 0..16 {
                                        for y in 0..16 {
                                            for z in 0..16 {
                                                if ray.0 > (x as f32)-0.5 && ray.0 < (x as f32)+0.5
                                                && ray.1 > (y as f32)-0.5 && ray.1 < (y as f32)+0.5
                                                && ray.2 > (z as f32)-0.5 && ray.2 < (z as f32)+0.5 {
                                                    println!("Chunk intersection with block at ({}, {}, {}): {}", x, y, z, chunk[x][y][z].id);
                                                    if chunk[x][y][z].id != 0 {
                                                        hit = true;
                                                        chunk[x][y][z] = Block::default();
                                                        mesh_vertices = meshgen::gen_chunk_mesh(&chunk);
                                                        mesh = mesh::Mesh::new(mesh_vertices, &mesh_texture, &shader);
                                                        steps = hit_radius;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    ray.0 += 0.25 * camera.forward[0];
                                    ray.1 += 0.25 * camera.forward[1];
                                    ray.2 += 0.25 * camera.forward[2];
                                    steps += 1;
                                }
                            }
                        },
                        _ => {}
                    }
                },
                glfw::WindowEvent::Key(key  , code, action, modifiers) => match key {
                    glfw::Key::Escape => window.set_should_close(true),
                    glfw::Key::W => camera.move_direction(Vector3::new(0.0, 0.0, 1.0)),
                    glfw::Key::S => camera.move_direction(Vector3::new(0.0, 0.0, -1.0)),
                    glfw::Key::D => camera.move_direction(Vector3::new(1.0, 0.0, 0.0)),
                    glfw::Key::A => camera.move_direction(Vector3::new(-1.0, 0.0, 0.0)),
                    glfw::Key::Space => camera.move_direction(Vector3::new(0.0, 1.0, 0.0)),
                    glfw::Key::LeftShift => camera.move_direction(Vector3::new(0.0, -1.0, 0.0)),
                    _ => {}
                },
                _ => {}
            }
        }
        
    }

}