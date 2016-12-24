#[macro_use]
extern crate glium;
extern crate libc;
extern crate nalgebra as na;
extern crate rand;

mod vertex;
mod treenode;
mod libcaca_ffi;

// use vertex::Vertex;

// const TRIANGLE: [Vertex; 3] = [
//     Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
//     Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
//     Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0] },
// ];

const SCREEN_HEIGHT: u32 = 800;
const SCREEN_WIDTH: u32 = 600;

fn create_dither(width: u32, height: u32) -> Result<*mut libcaca_ffi::caca_dither_t, String> {
    let dither = unsafe {
        #[cfg(target_endian = "little")]
        fn get_masks() -> (u32, u32, u32, u32) { (0xff, 0xff00, 0xff0000, 0xff000000) }
        #[cfg(target_endian = "big")]
        fn get_masks() -> (u32, u32, u32, u32) { (0xff000000, 0xff0000, 0xff00, 0xff) }

        let masks = get_masks();
        libcaca_ffi::caca_create_dither(32, width as libc::c_int,
                                        height as libc::c_int,
                                        width as libc::c_int * 4,
                                        masks.0, masks.1, masks.2, masks.3)
    };

    if dither.is_null() {
        return Err("caca_create_dither failed".to_string());
    }

    Ok(dither)
}

fn init_caca() -> Result<(*mut libcaca_ffi::caca_display_t), String> {
    let display = unsafe { libcaca_ffi::caca_create_display(std::ptr::null_mut()) };
    if display.is_null() {
        return Err("Could not create libcaca display".to_string());
    }

    Ok(display)
}

fn load_shader_source(path: &str) -> Result<String, std::io::Error> {
    use std::io::prelude::*;
    use std::fs::File;

    let mut file = try!(File::open(path));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    Ok(contents)
}

fn main() {
    use glium::{DisplayBuild, Surface};

    let start_pos = na::Vector3::new(1.0, 1.0, 1.0);
    let start_rot = na::Vector3::new(0.0, 1.0, 0.0);
    let mut n = treenode::TreeNode::new(2, start_pos, start_rot);
    n.build();
    // treenode::print_tree(&n);

    let caca_display = init_caca().unwrap();

    let vert_shader = load_shader_source("shader/triangle_150.vs").unwrap();
    let frag_shader = load_shader_source("shader/triangle_150.fs").unwrap();

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .build_glium()
        .unwrap();

    let mut triangles = treenode::make_triangles(&n, 1.0, 0.0, 0.0);
    triangles.append(&mut treenode::make_triangles(&n, 0.8, 0.3, 1.0));
    triangles.append(&mut treenode::make_triangles(&n, 0.6, 0.5, 1.5));
    triangles.append(&mut treenode::make_triangles(&n, 0.4, 0.7, 0.2));
    // let vertex_buffer = glium::VertexBuffer::new(&display, &TRIANGLE).unwrap();
    let vertex_buffer = glium::VertexBuffer::new(&display, &triangles).unwrap();
    let indinces = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = glium::Program::from_source(&display, &vert_shader, &frag_shader, None)
        .unwrap();

    let mut running = true;
    while running {
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => running = false,
                _ => ()
            }
        }

        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indinces, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        let buffer: glium::texture::RawImage2d<u8> = display.read_front_buffer();
        let buffer_raw = buffer.data.into_owned();
        let dither = create_dither(buffer.width, buffer.height).unwrap();

        let mut buffer_data = vec![0; (buffer.width * buffer.height) as usize];
        for x in 0..buffer.width {
            for y in 0..buffer.height {
                let i = (((buffer.height - y - 1) * buffer.width + x) * 4) as usize;
                let new_i = (y * buffer.width + x) as usize;
                let a = buffer_raw[i] as u32;
                let b = buffer_raw[i + 1] as u32;
                let g = buffer_raw[i + 2] as u32;
                let r = buffer_raw[i + 3] as u32;
                buffer_data[new_i] =
                    r << 24 | g << 16 | b << 8 | a;
            }
        }

        unsafe {
            let canvas = libcaca_ffi::caca_get_canvas(caca_display);
            let caca_width = libcaca_ffi::caca_get_canvas_width(canvas);
            let caca_height = libcaca_ffi::caca_get_canvas_height(canvas);

            libcaca_ffi::caca_dither_bitmap(canvas, 0, 0, caca_width, caca_height, dither,
                                            buffer_data.as_ptr() as *const _);
            libcaca_ffi::caca_refresh_display(caca_display);
            libcaca_ffi::caca_free_dither(dither);
        }
    }

    unsafe {
        libcaca_ffi::caca_free_display(caca_display);
    }
}

