extern crate gl;
extern crate glutin;
extern crate libc;

use glutin::GlContext;

fn main() {
    //let window = glutin::Window::new().unwrap();
    let window = glutin::api::caca::Window::new(
        glutin::WindowAttributes::default()
    ).unwrap();

    //unsafe { window.make_current().unwrap() };

    unsafe {
        //gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    for event in window.wait_events() {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
        window.swap_buffers().unwrap();

        match event {
            glutin::Event::Closed => break,
            _ => ()
        }
    }
}
