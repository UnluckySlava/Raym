#![windows_subsystem = "windows"]

extern crate gl;
extern crate sdl2;

mod camera_mod;
mod vector_math;
mod scene;

use camera_mod::Camera;
use gl::types::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::video::GLProfile;
use std::collections::HashMap;
use std::ffi::CString;
use std::fs;
use scene::*;
use vector_math::Vector3;

const MOUSE_SENSITIVITY: f32 = 0.002;

fn main() {
    // Making the main window
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Window", 800, 600)
        .opengl()
        .build()
        .unwrap();

    let _ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 3));
    ////////////////////////////////////////////////////////////
    //screen surface
    let vertex_array: [f32; 18] = [
        // first triangle
        1.0, 1.0, 0.0, // top right
        1.0, -1.0, 0.0, // bottom right
        -1.0, 1.0, 0.0, // top left
        // second triangle
        1.0, -1.0, 0.0, // bottom right
        -1.0, -1.0, 0.0, // bottom left
        -1.0, 1.0, 0.0, // top left
    ];
    let mut vbo: GLuint = 0;
    let mut vao: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (std::mem::size_of::<f32>() * vertex_array.len()) as GLsizeiptr,
            vertex_array.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (std::mem::size_of::<f32>() * 3) as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
    }
    ////////////////////////////////////////////////////////////
    // Making the shader program
    let vertex_source = fs::read_to_string(r"shaders\vertex.glsl").unwrap();
    let fragment_source = fs::read_to_string(r"shaders\fragment.glsl").unwrap();
    let shader_program: GLuint = make_program(vertex_source, fragment_source);
    ////////////////////////////////////////////////////////////
    // Program uniforms
    let uniform_resolution =
        unsafe { gl::GetUniformLocation(shader_program, "resolution\0".as_ptr() as *const i8) };
    let resolution = window.size();
    unsafe {
        gl::UseProgram(shader_program);
        gl::Uniform2f(uniform_resolution, resolution.0 as f32, resolution.1 as f32);
    }
    let uniform_camera_pos =
        unsafe { gl::GetUniformLocation(shader_program, "cameraPos\0".as_ptr() as *const i8) };
    let uniform_camera_rot =
        unsafe { gl::GetUniformLocation(shader_program, "cameraRot\0".as_ptr() as *const i8) };

    
    let mut scene = Scene {
        objects: HashMap::new()
    };
    scene.objects.insert(String::from("Sphere"), Box::new(Sphere{pos: Vector3::new(0.0, 2.0, 0.0), radius: 2.0}));
    //
    // Main gameloop
    let mut camera = Camera {
        pos: Vector3::new(0.0, 0.0, 5.0),
        rot: Vector3::new(0.0, 0.0, 0.0),
    };
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion {
                    yrel: i,
                    xrel: j,
                    mousestate: mouse,
                    ..
                } => {
                    if mouse.left() {
                        camera.rot.x -= i as f32 * MOUSE_SENSITIVITY;
                        camera.rot.y += j as f32 * MOUSE_SENSITIVITY;
                    }
                }
                _ => {}
            }
        }
        let mut dir = vector_math::Vector3::new(0.0, 0.0, 0.0);
        for key in event_pump.keyboard_state().scancodes() {
            match key {
                (Scancode::W, true) => dir.z -= 1.0,
                (Scancode::S, true) => dir.z += 1.0,
                (Scancode::A, true) => dir.x -= 1.0,
                (Scancode::D, true) => dir.x += 1.0,
                (Scancode::Space, true) => dir.y += 1.0,
                (Scancode::LCtrl, true) => dir.y -= 1.0,
                _ => {}
            }
        }
        camera.control(dir * 0.1);

        // render the image
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program);
            gl::Uniform3f(
                uniform_camera_rot
        ,
                camera.rot.x,
                camera.rot.y,
                camera.rot.z,
            );
            gl::Uniform3f(uniform_camera_pos, camera.pos.x, camera.pos.y, camera.pos.z);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
        window.gl_swap_window();
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn make_shader(shader_source: String, shader_type: GLenum) -> u32 {
    let shader_source = CString::new(shader_source).unwrap();
    let tmp: *const *const GLchar = &shader_source.as_ptr();
    unsafe {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, tmp, std::ptr::null());
        gl::CompileShader(shader);
        let mut success: i32 = 0;
        let mut infolog: [u8; 512] = [0; 512];
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != 1 {
            gl::GetShaderInfoLog(
                shader,
                512,
                std::ptr::null_mut(),
                infolog.as_mut_ptr() as *mut i8,
            );
            println!(
                "{}", String::from_utf8_lossy(&infolog).trim_end_matches('\0')
            );
            panic!("{} shader, compilation failed", {
                if shader_type == gl::VERTEX_SHADER {
                    "Vertex "
                } else if shader_type == gl::FRAGMENT_SHADER {
                    "Fragment"
                } else {"Unknown"}
            });
        }
        shader
    }
}

fn make_program(vertex_source: String, fragment_source: String) -> u32 {
    let vertex_shader = make_shader(vertex_source, gl::VERTEX_SHADER);
    let fragment_shader = make_shader(fragment_source, gl::FRAGMENT_SHADER);
    let shader_program: u32;
    unsafe {
        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }
    shader_program
}
