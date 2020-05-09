extern crate gl;
extern crate sdl2;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("Demo", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    // GL functions need to be loaded with gl_get_proc_address since the GL library is loaded at
    // runtime.
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::ffi::c_void);
    unsafe {
        gl::ClearColor(0.4, 0.6, 0.4, 1.0);
    }
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { timestamp } => {
                    println!("Quit at {}", timestamp);
                    break 'main;
                }
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
    }
}
