mod camera;
mod cube;
mod vectormath;
mod vertex;
mod meshgen;
mod mesh;
mod block;
mod shader;
mod macros;
mod player;

use block::{BLOCKS, Block};
use camera::*;
use cgmath::{Matrix4, Vector3};
use glfw::Context;
use mesh::{Texture, texture_from_file};
use meshgen::gen_chunk_mesh;
use vectormath::dda;
use std::{ffi::CStr, vec};

extern crate image;
extern crate glfw;
extern crate gl;

const _SCENE_LIGHT: [f32; 3] = [-1.0, 0.701, -1.0];

const WIDTH: u32 = 960;
const HEIGHT: u32 = 720;

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

    
    
    //let chunks = Vec::with_capacity(5);
    //for i in 0..5 {
        let mut chunk: [[[usize; 16]; 16]; 16] = [[[0; 16]; 16]; 16];
        chunk[0][0][0] = 1;
        for x in 1..16 {
            for y in 1..16 {
                for z in 1..16 {
                    if (y as f32) < ((x*x + z*z) as f32).sqrt() {
                        if y < 3 {
                            chunk[x][y][z] = 1;
                        } else if y < 4 {
                            chunk[x][y][z] = 3;
                        } else if y < 5 {
                            chunk[x][y][z] = 2;
                        } else {
                            chunk[x][y][z] = 0;
                        }
                    }
                }
            }
        }
        //chunks.push(chunk);
    //}

    let texture_id = texture_from_file("terrain.png", ".");
    let mesh_texture = Texture {id: texture_id};
    let mut mesh_vertices = gen_chunk_mesh(&chunk);
    let mut mesh = mesh::Mesh::new(mesh_vertices, &mesh_texture, &shader);

    let mut cursor_cube = [[[0usize; 16]; 16]; 16];
    cursor_cube[0][0][0] = 1;
    let cursor_cube_verts = gen_chunk_mesh(&cursor_cube);
    let cursor_cube_mesh = mesh::Mesh::new(cursor_cube_verts, &mesh_texture, &shader);
    let mut cursor_position: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

    //let mut camera = Camera::new(Vector3::new(-8.0, 11.0, -9.0), Vector3::new(0.568056107, -0.487900823, 0.662770748));
    let mut player = player::Player::new(Vector3::new(5.0, 5.8, 4.5), Vector3::new(1.0, 0.0, 0.0));
    //camera.set_move_speed(0.5);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    
    let mut previous_time = glfw.get_time();
    let mut frame_count = 0;
    while !window.should_close() {
        let current_time = glfw.get_time();
        frame_count += 1;
        if current_time - previous_time >= 1.0 {
            // Display the frame count here any way you want.
            println!("FPS: {}", frame_count);

            frame_count = 0;
            previous_time = current_time;
        }
        std::thread::sleep(std::time::Duration::from_nanos(11111111));

        player.update(&chunk);
        /*if let Some((intersect, block, )) = dda(&chunk, &player.delta, &player.direction, vectormath::len(&player.delta)) {
            cursor_position = intersect;
        }*/
        //cursor_position = player.position + player.delta;

        unsafe {
            
            gl::ClearColor(0.1, 0.4, 0.95, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader.use_program();

            let projection: Matrix4<f32> = perspective_matrix();//cgmath::perspective(cgmath::Deg(90.0), WIDTH as f32 / HEIGHT as f32, 0.1, 100.0);
            let view = player.camera.view_matrix();
            let model = Matrix4::from_scale(1.0);

            shader.set_mat4(c_str!("perspective_matrix"), &projection);
            shader.set_mat4(c_str!("view_matrix"), &view);
            shader.set_mat4(c_str!("model_matrix"), &model);
            mesh.draw();

            //let cursor_projection: Matrix4<f32> = perspective_matrix();//cgmath::perspective(cgmath::Deg(90.0), WIDTH as f32 / HEIGHT as f32, 0.1, 100.0);
            //let view = player.camera.view_matrix();
            let cursor_model = Matrix4::from_translation(cursor_position);
            shader.set_mat4(c_str!("model_matrix"), &cursor_model);

            cursor_cube_mesh.draw();

        }
        
        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::CursorPos(x, y) => {
                    let delta = (x-WIDTH as f64/2.0, y-HEIGHT as f64/2.0);
                    window.set_cursor_pos(WIDTH as f64/2.0, HEIGHT as f64/2.0);
                    player.camera.rotate_on_x_axis(0.001 * delta.1 as f32);
                    player.camera.rotate_on_y_axis(0.001 * delta.0 as f32);
                },
                glfw::WindowEvent::MouseButton(button, action, _modifiers) => {
                    match button {
                        glfw::MouseButton::Button1 => {
                            if action == glfw::Action::Press {
                                if let Some((chunk_index, block_index)) = dda(&chunk, &player.camera.position, &player.camera.forward, 6.0) {
                                    chunk[block_index.x as usize][block_index.y as usize][block_index.z as usize] = 0;
                                    mesh_vertices = meshgen::gen_chunk_mesh(&chunk);
                                    mesh = mesh::Mesh::new(mesh_vertices, &mesh_texture, &shader);
                                    //println!("block hit: {:?}", block_index);
                                }
                            }
                        },
                        glfw::MouseButton::Button2 => {
                            if action == glfw::Action::Press {
                                if let Some((adjacent, block_index)) = dda(&chunk, &player.camera.position, &player.camera.forward, 6.0) {
                                    chunk[adjacent.x as usize][adjacent.y as usize][adjacent.z as usize] = 3;
                                    mesh_vertices = meshgen::gen_chunk_mesh(&chunk);
                                    mesh = mesh::Mesh::new(mesh_vertices, &mesh_texture, &shader);
                                    //println!("block hit: {:?}", block_index);
                                }
                            }
                        },
                        _ => {}
                    }
                },
                glfw::WindowEvent::Key(key  , _code, action, _modifiers) => match key {
                    glfw::Key::Escape => window.set_should_close(true),
                    glfw::Key::W =>  { if action == glfw::Action::Press { player.move_direction(Vector3::new(0.0, 0.0, 1.0)) } else if action == glfw::Action::Release { player.stop_move_direction(Vector3::new(0.0, 0.0, 1.0)) } },
                    glfw::Key::S =>  { if action == glfw::Action::Press { player.move_direction(Vector3::new(0.0, 0.0, -1.0)) } else if action == glfw::Action::Release { player.stop_move_direction(Vector3::new(0.0,0.0, -1.0)) } },
                    glfw::Key::D =>  { if action == glfw::Action::Press { player.move_direction(Vector3::new(1.0, 0.0, 0.0)) } else if action == glfw::Action::Release { player.stop_move_direction(Vector3::new(1.0, 0.0, 0.0)) } },
                    glfw::Key::A =>  { if action == glfw::Action::Press { player.move_direction(Vector3::new(-1.0, 0.0, 0.0)) } else if action == glfw::Action::Release { player.stop_move_direction(Vector3::new(-1.0, 0.0, 0.0)) } },
                    glfw::Key::Space =>  { if action == glfw::Action::Press { player.move_direction(Vector3::new(0.0, 2.0, 0.0)) } },
                    //glfw::Key::LeftShift =>  { if action == glfw::Action::Press { player.move_direction(Vector3::new(0.0, -1.0, 0.0)) } else if action == glfw::Action::Release { player.stop_move_direction(Vector3::new(0.0, -1.0, 0.0)) } },
                    _ => {}
                },
                _ => {}
            }
        }
        
    }

}