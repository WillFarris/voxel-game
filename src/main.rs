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
mod inventory;
mod collision;

use camera::*;
use cgmath::{Matrix4, Vector2, Vector3};
use glfw::Context;
use mesh::texture_from_file;
use vectormath::dda;
use std::ffi::CStr;

use crate::vertex::Vertex;

extern crate image;
extern crate glfw;
extern crate gl;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    #[cfg(target_arch = "arm")] {
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

    let block_vertex_shader_path = if cfg!(target_arch = "arm") {
        "shaders/block_vertex_es.glsl"
    } else {
        "shaders/block_vertex.glsl"
    };
    let grass_vertex_shader_path = if cfg!(target_arch = "arm") {
        "shaders/grass_vertex_es.glsl"
    } else {
        "shaders/grass_vertex.glsl"
    };
    let leaves_vertex_shader_path = if cfg!(target_arch = "arm") {
        "shaders/leaves_vertex_es.glsl"
    } else {
        "shaders/leaves_vertex.glsl"
    };

    let block_fragment_shader_path = if cfg!(target_arch = "arm") {
        "shaders/block_fragment_es.glsl"
    } else {
        "shaders/block_fragment.glsl"
    };

    let block_shader = shader::Shader::new(block_vertex_shader_path, block_fragment_shader_path);
    let grass_shader = shader::Shader::new(grass_vertex_shader_path, block_fragment_shader_path);
    let leaves_shader = shader::Shader::new(leaves_vertex_shader_path, block_fragment_shader_path);
    
    let gui_shader = shader::Shader::new("shaders/gui_vertex.glsl", "shaders/gui_fragment.glsl");

    //let mut cube1 = cube::Cube::new([-1.0, 5.0, 5.0], [0.9, 0.2, 0.2]);

    let terrain_texture_id = texture_from_file("terrain.png", ".");
    let seed = rand::random();
    let mut world = world::World::new(mesh::Texture{id: terrain_texture_id}, &block_shader, &grass_shader, &leaves_shader, seed);

    let mut player = player::Player::new(Vector3::new(5.0, 65.0, 4.5), Vector3::new(1.0, 0.0, 1.0));

    let inventory_texture_id = texture_from_file("gui.png", ".");
    let gui_verts = Vec::from([   
        Vertex { position: Vector3::new( -0.5,  -0.8,  0.0), normal: Vector3::new( 0.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.9140625) },
        Vertex { position: Vector3::new( 0.5, -0.8, 0.0), normal: Vector3::new( 0.0,  0.0, 0.0), tex_coords: Vector2::new(0.7109375, 0.9140625) },
        Vertex { position: Vector3::new( 0.5,  -1.0, 0.0), normal: Vector3::new( 0.0,  0.0, 0.0), tex_coords: Vector2::new(0.7109375, 1.0) },

        Vertex { position: Vector3::new( -0.5, -0.8,  0.0), normal: Vector3::new( 0.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 0.9140625) },
        Vertex { position: Vector3::new( 0.5, -1.0, 0.0), normal: Vector3::new( 0.0,  0.0, 0.0), tex_coords: Vector2::new(0.7109375, 1.0) },
        Vertex { position: Vector3::new( -0.5,  -1.0,  0.0), normal: Vector3::new( 0.0,  0.0, 0.0), tex_coords: Vector2::new(0.0, 1.0) },
        
    ],);
    let gui_mesh = mesh::Mesh::new(gui_verts, &mesh::Texture { id: inventory_texture_id}, &gui_shader);


    let sunlight_direction: Vector3<f32> = Vector3 { x: -0.701, y: 0.701, z: -0.701 };
    

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
        gl::FrontFace(gl::CW);

        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
    
    let start_time = glfw.get_time();
    let mut previous_time = start_time;
    while !window.should_close() {
        let current_time = glfw.get_time();
        let delta_time = (current_time - previous_time) as f32;
        let elapsed_time = current_time - start_time;
        previous_time = current_time;
        player.update(&world, delta_time);

        unsafe {          
            gl::ClearColor(0.1, 0.4, 0.95, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            //solid_shader.use_program();
            //transparent_shader.use_program();

            let projection: Matrix4<f32> = perspective_matrix();//cgmath::perspective(cgmath::Deg(90.0), WIDTH as f32 / HEIGHT as f32, 0.1, 100.0);
            let view = player.camera.view_matrix();
            //let model = Matrix4::from_scale(1.0);

            block_shader.use_program();
            block_shader.set_mat4(c_str!("perspective_matrix"), &projection);
            block_shader.set_mat4(c_str!("view_matrix"), &view);
            block_shader.set_vec3(c_str!("sunlight_direction"), &sunlight_direction);
            block_shader.set_float(c_str!("time"), elapsed_time as f32);
            world.render_solid(player.position, player.camera.forward);
            
            grass_shader.use_program();
            grass_shader.set_mat4(c_str!("perspective_matrix"), &projection);
            grass_shader.set_mat4(c_str!("view_matrix"), &view);
            grass_shader.set_vec3(c_str!("sunlight_direction"), &sunlight_direction);
            grass_shader.set_float(c_str!("time"), elapsed_time as f32);
            world.render_grass();

            leaves_shader.use_program();
            leaves_shader.set_mat4(c_str!("perspective_matrix"), &projection);
            leaves_shader.set_mat4(c_str!("view_matrix"), &view);
            leaves_shader.set_vec3(c_str!("sunlight_direction"), &sunlight_direction);
            leaves_shader.set_float(c_str!("time"), elapsed_time as f32);
            world.render_leaves();

            gui_shader.use_program();
            gui_shader.set_float(c_str!("selected"), (player.inventory.selected as f32 - 1.0).max(0.0));
            gui_mesh.draw(&gui_shader);

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
                                if let Some((intersect_position, world_index)) = dda(&world, &player.camera.position, &player.camera.forward, 6.0) {
                                    let place_index = Vector3 {
                                        x: if intersect_position.x == world_index.x as f32 {
                                            world_index.x - 1
                                        } else if intersect_position.x-1.0 == world_index.x as f32 {
                                            world_index.x + 1
                                        } else {
                                            world_index.x
                                        },
                                        y: if intersect_position.y== world_index.y as f32 {
                                            world_index.y - 1
                                        } else if intersect_position.y-1.0 == world_index.y as f32 {
                                            world_index.y + 1
                                        } else {
                                            world_index.y
                                        },
                                        z: if intersect_position.z == world_index.z as f32 {
                                            world_index.z - 1
                                        } else if intersect_position.z-1.0 == world_index.z as f32 {
                                            world_index.z + 1
                                        } else {
                                            world_index.z
                                        },
                                    };
                                    world.place_at_global_pos(place_index, player.inventory.selected);
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
                    glfw::Key::Space =>  { if action == glfw::Action::Press { player.move_direction(Vector3::new(0.0, 1.5, 0.0)) } else if action == glfw::Action::Release { /*player.stop_move_direction(Vector3::new(0.0, 2.0, 0.0))*/ } },
                    //glfw::Key::LeftShift =>  { if action == glfw::Action::Press { player.move_direction(Vector3::new(0.0, -1.0, 0.0)) } else if action == glfw::Action::Release { player.stop_move_direction(Vector3::new(0.0, -1.0, 0.0)) } },
                    
                    glfw::Key::Num1 => { if action == glfw::Action::Press { player.inventory.selected = 1;} }
                    glfw::Key::Num2 => { if action == glfw::Action::Press { player.inventory.selected = 2;} }
                    glfw::Key::Num3 => { if action == glfw::Action::Press { player.inventory.selected = 3;} } 
                    glfw::Key::Num4 => { if action == glfw::Action::Press { player.inventory.selected = 4;} }
                    glfw::Key::Num5 => { if action == glfw::Action::Press { player.inventory.selected = 5;} }
                    glfw::Key::Num6 => { if action == glfw::Action::Press { player.inventory.selected = 6;} }
                    glfw::Key::Num7 => { if action == glfw::Action::Press { player.inventory.selected = 7;} }
                    glfw::Key::Num8 => { if action == glfw::Action::Press { player.inventory.selected = 8;} }
                    glfw::Key::Num9 => { if action == glfw::Action::Press { player.inventory.selected = 9;} }
                    glfw::Key::Num0 => { if action == glfw::Action::Press { player.inventory.selected = 10;} }
                    
                    _ => {}
                },
                _ => {}
            }
        }
        
    }

}
