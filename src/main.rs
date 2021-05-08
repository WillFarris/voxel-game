mod camera;
mod cube;
mod vectormath;
mod vertex;

use camera::*;
use vectormath::quaternion_rotation_matrix;
use vertex::*;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern crate glium;
use glium::{
    glutin::{self, event::VirtualKeyCode},
    uniform, DrawParameters, IndexBuffer, Program, Surface, VertexBuffer,
};

extern crate image;

const SCENE_LIGHT: [f32; 3] = [1.0, 1.0, -1.0];

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

    let teapot_shader = glium::Program::from_source(
        &display,
        tri_vertex_shader_str.as_str(),
        tri_fragment_shader_str.as_str(),
        None,
    )
    .unwrap();

    let mut cube1 = cube::Cube::new([-1.0, 1.0, 5.0], &display, None, [0.6, 0.2, 0.2]);
    let mut cube2 = cube::Cube::new([1.0, 0.0, 5.0], &display, None, [0.22, 0.6, 0.1]);
    let mut camera = Camera::new(&[0.0, 0.0, 0.0], &[0.0, 0.0, 5.0]);

    let mut t: f32= 0.0;
    event_loop.run(move |ev, _, control_flow| {
        t += 0.001;
        let mut target = display.draw();

        target.clear_color_and_depth((0.1, 0.15, 0.9, 1.0), 1.0);


        //cube.translate(&[0.005 * (t/3.0).cos(), 0.0, 0.005 * (t/3.0).sin()]);
        /*let r = quaternion_rotation_matrix(&[0.0, 1.0, 0.0], t);
        for x in 0..3 {
            for y in 0..3 {
                cube.model_matrix[x][y] = r[x][y];
            }
        }*/

        cube1.model_matrix[0][0] = t.cos();
        cube1.model_matrix[0][2] = t.sin();
        cube1.model_matrix[2][0] = -t.sin();
        cube1.model_matrix[2][2] = t.cos();

        cube2.model_matrix[0][0] = (-t).cos();
        cube2.model_matrix[0][2] = (-t).sin();
        cube2.model_matrix[2][0] = t.sin();
        cube2.model_matrix[2][2] = (-t).cos();

        cube1.draw(&mut target, &params, &camera, &teapot_shader);
        cube2.draw(&mut target, &params, &camera, &teapot_shader);

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
                            match code {
                                VirtualKeyCode::W => camera.move_direction(&[0.0, 0.0, 0.5]),
                                VirtualKeyCode::S => camera.move_direction(&[0.0, 0.0, -0.5]),
                                VirtualKeyCode::A => camera.move_direction(&[-0.5, 0.0, 0.0]),
                                VirtualKeyCode::D => camera.move_direction(&[0.5, 0.0, 0.0]),
                                VirtualKeyCode::Space => camera.move_direction(&[0.0, 0.5, 0.0]),
                                VirtualKeyCode::LShift => camera.move_direction(&[0.0, -0.5, 0.0]),

                                VirtualKeyCode::C => t += 0.05,

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
