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
mod world;

use camera::*;
use cgmath::{Matrix4, Vector3};
use glfw::Context;
use mesh::texture_from_file;
use vectormath::dda;
use world::CHUNK_SIZE;
use std::ffi::CStr;

use noise::{NoiseFn, Perlin, Seedable};

extern crate image;
extern crate glfw;
extern crate gl;

const _SCENE_LIGHT: [f32; 3] = [-1.0, 0.701, -1.0];

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1280;

fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    #[cfg(target_arch = "aarch64")] {
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 1));
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::OpenGlEs));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    }

    let (mut window, events) = glfw.create_window(WIDTH, HEIGHT, "", glfw::WindowMode::Windowed).expect("Failed to create GLFW window");


    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos(WIDTH as f64/2.0, HEIGHT as f64/2.0);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let world_vertex_shader_path = if cfg!(aarch64) {
        "shaders/cube_vertex_es.glsl"
    } else {
        "shaders/cube_vertex.glsl"
    };
    let world_fragment_shader_path = if cfg!(aarch64) {
        "shaders/cube_fragment_es.glsl"
    } else {
        "shaders/cube_fragment.glsl"
    };
    let world_shader = shader::Shader::new(world_vertex_shader_path, world_fragment_shader_path);

    //let mut cube1 = cube::Cube::new([-1.0, 5.0, 5.0], [0.9, 0.2, 0.2]);

    let texture_id = texture_from_file("terrain.png", ".");
    let mut world = world::World::new(texture_id, &world_shader);

    let perlin = Perlin::new();
    perlin.set_seed(rand::random());

    let noise_x_offset = rand::random::<f64>();
    let noise_z_offset = rand::random::<f64>();
    let noise_scale = 0.03;

    let num_square_chunks = 10;
    for chunk_x in 0..num_square_chunks {
        for chunk_z in 0..num_square_chunks {
            let mut cur_chunk: [[[usize; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] = [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
            for block_x in 0..CHUNK_SIZE {
                for block_y in 0..CHUNK_SIZE {
                    for block_z in 0..CHUNK_SIZE {
                        let global_x = noise_scale * (block_x + (chunk_x * CHUNK_SIZE)) as f64;
                        let global_z = noise_scale * (block_z + (chunk_z * CHUNK_SIZE)) as f64;
                        let max_y = {
                            let noise_val = (10.0 * (perlin.get([global_x + noise_x_offset, global_z + noise_z_offset])+1.0)/2.0 + 1.0) as usize;

                            //noise_val += (5.0 * (perlin.get([2.0 * global_x + noise_x_offset, 2.0 * global_z + noise_z_offset])+1.0)/2.0 + 1.0) as usize;

                            noise_val
                        };
                        if block_y < max_y {
                            if block_y == max_y-1 {
                                cur_chunk[block_x][block_y][block_z] = 2;
                            } else if block_y < max_y/2 {
                                cur_chunk[block_x][block_y][block_z] = 1;
                            } else {
                                cur_chunk[block_x][block_y][block_z] = 3;
                            }
                        }
                    }
                }
            }
            world.chunk_from_block_array(Vector3::new(chunk_x as isize, 0, chunk_z as isize), cur_chunk);
        }
    }
    

    /*let mut cursor_cube = [[[0usize; 16]; 16]; 16];
    cursor_cube[0][0][0] = 1;
    let cursor_cube_verts = gen_chunk_mesh(&cursor_cube);
    let cursor_cube_mesh = mesh::Mesh::new(cursor_cube_verts, &mesh_texture, &world_shader);
    let mut cursor_position: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);*/

    //let mut camera = Camera::new(Vector3::new(-8.0, 11.0, -9.0), Vector3::new(0.568056107, -0.487900823, 0.662770748));
    let mut player = player::Player::new(Vector3::new(5.0, 10.5, 4.5), Vector3::new(1.0, 0.0, 1.0));
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

        player.update(&world);
        /*if let Some((intersect, block, )) = dda(&chunk, &player.delta, &player.direction, vectormath::len(&player.delta)) {
            cursor_position = intersect;
        }*/
        //cursor_position = player.position + player.delta;

        unsafe {
            
            gl::ClearColor(0.1, 0.4, 0.95, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            world_shader.use_program();

            let projection: Matrix4<f32> = perspective_matrix();//cgmath::perspective(cgmath::Deg(90.0), WIDTH as f32 / HEIGHT as f32, 0.1, 100.0);
            let view = player.camera.view_matrix();
            //let model = Matrix4::from_scale(1.0);

            world_shader.set_mat4(c_str!("perspective_matrix"), &projection);
            world_shader.set_mat4(c_str!("view_matrix"), &view);
            world.render(&projection, &view);

            //let cursor_projection: Matrix4<f32> = perspective_matrix();//cgmath::perspective(cgmath::Deg(90.0), WIDTH as f32 / HEIGHT as f32, 0.1, 100.0);
            //let view = player.camera.view_matrix();
            /*let cursor_model = Matrix4::from_translation(cursor_position);
            world_shader.set_mat4(c_str!("model_matrix"), &cursor_model);

            cursor_cube_mesh.draw();*/

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
                                if let Some((_, world_index)) = dda(&world, &player.camera.position, &player.camera.forward, 6.0) {
                                    world.destroy_at_global_pos(world_index);
                                }
                            }
                        },
                        glfw::MouseButton::Button2 => {
                            if action == glfw::Action::Press {
                                if let Some((_, world_index)) = dda(&world, &player.camera.position, &player.camera.forward, 6.0) {
                                    world.place_at_global_pos(world_index, 4);
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