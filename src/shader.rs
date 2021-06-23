use std::{ffi::CStr, io::prelude::*};
use std::fs::File;
use std::path::Path;
use std::{ffi::CString, ptr};

use cgmath::{Array, Matrix, Matrix4, Vector3};
use gl::types::*;

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {

        let mut shader_program: Shader = Shader { id: 0u32 };

        let mut vertex_shader_file = match File::open(Path::new(vertex_path)) {
            Err(why) => panic!("Could not open file: {}", why),
            Ok(file) => file,
        };
        let mut vertex_shader_string = String::new();
        match vertex_shader_file.read_to_string(&mut vertex_shader_string) {
            Err(why) => panic!("Could not read file: {}", why),
            Ok(_) => (),
        }
        let mut fragment_shader_file = match File::open(Path::new(fragment_path)) {
            Err(why) => panic!("Could not open file: {}", why),
            Ok(file) => file,
        };
        let mut fragment_shader_string = String::new();
        match fragment_shader_file.read_to_string(&mut fragment_shader_string) {
            Err(why) => panic!("Could not read file: {}", why),
            Ok(_) => (),
        }
        unsafe {
            // build and compile our shader program
            // ------------------------------------
            // vertex shader
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_str_vert = CString::new(vertex_shader_string.as_bytes()).unwrap();
            gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
    
            // check for shader compile errors
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(vertex_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", std::str::from_utf8(&info_log).unwrap());
            }
    
            // fragment shader
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_str_frag = CString::new(fragment_shader_string.as_bytes()).unwrap();
            gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            // check for shader compile errors
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(fragment_shader, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", std::str::from_utf8(&info_log).unwrap());
            }
    
            // link shaders
            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader);
            gl::AttachShader(program_id, fragment_shader);
            gl::LinkProgram(program_id);
            // check for linking errors
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(program_id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", std::str::from_utf8(&info_log).unwrap());
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            shader_program.id = program_id;
        }
        shader_program
    }

    pub fn set_mat4(&self, name: &CStr, mat: &Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, gl::FALSE, mat.as_ptr());
        }
    }

    pub fn set_vec3(&self, name: &CStr, vec: &Vector3<f32>) {
        unsafe {
            gl::Uniform3fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, vec.as_ptr());
        }
    }

    pub fn set_float(&self, name: &CStr, float: f32) {
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), float);
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}