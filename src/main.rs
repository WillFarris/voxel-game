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
use meshgen::gen_chunk_mesh;
use std::ffi::{CString, CStr};
use vertex::Vertex;

use std::sync::mpsc::Receiver;



extern crate image;
extern crate glfw;
extern crate gl;

const SCENE_LIGHT: [f32; 3] = [-1.0, 0.701, -1.0];



fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(800, 600, "", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");


    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    //window.set_cursor_pos(400.0, 300.0);
    //window.set_cursor_mode(glfw::CursorMode::Hidden);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let shader = shader::Shader::new("shaders/cube_vertex.glsl", "shaders/cube_fragment.glsl");


    //let mut cube1 = cube::Cube::new([-1.0, 5.0, 5.0], [0.9, 0.2, 0.2]);
    //let mut camera = Camera::new(&[8.0, 6.0, 0.0], &[0.0, 0.0, 1.0]);   
    let mut chunk: [[[Block; 16]; 16]; 16] = [[[Block::default(); 16]; 16]; 16];
    for x in 0..16 {
        for y in 0..4 {
            for z in 0..16 {
                chunk[x][y][z] = Block::new(1, Some(TextureType::Single(2,12)));
            }
        }
    }

    let mesh_vertices = gen_chunk_mesh(&chunk);
    let mesh = mesh::Mesh::new(mesh_vertices, Vec::new());

    let mut player_z = -10.0;
    while !window.should_close() {
        handle_events(&mut window, &events, &mut player_z);


        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            
            shader.use_program();

            let projection: Matrix4<f32> = cgmath::perspective(cgmath::Deg(90.0), 800f32 / 600f32, 0.1, 100.0);
            let view = cgmath::Matrix4::from_translation(Vector3::new(0.0, 0.0, player_z));
            let model = Matrix4::from_scale(1.0);

            shader.set_mat4(c_str!("perspective_matrix"), &projection);
            shader.set_mat4(c_str!("view_matrix"), &view);
            shader.set_mat4(c_str!("model_matrix"), &model);
            mesh.draw();

        }
        
        
        window.swap_buffers();
        glfw.poll_events();
        
    }

}

fn handle_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>, player_z: &mut f32) {
    for (_, event) in glfw::flush_messages(&events) {
        match event {
            glfw::WindowEvent::CursorPos(x, y) => {
                let delta = (x- 400.0, 300.0 - y);
                println!("Mouse: ({}, {})", delta.0, delta.1);
                //&CString::newwindow.set_cursor_pos(400.0, 300.0);
            },
            glfw::WindowEvent::MouseButton(button, action, modifiers) => {

            },
            glfw::WindowEvent::Key(key  , code, action, modifiers) => match key {
                glfw::Key::Escape => window.set_should_close(true),
                glfw::Key::W => *player_z += 0.5,
                glfw::Key::S => *player_z -= 0.5,
                _ => {}
            },
            _ => {}
        }
    }
}


/*
{
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
                                                    chunk[x][y][z] = Block::default();
                                                    mesh = meshgen::gen_chunk_mesh(&chunk);
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
                    }
*/