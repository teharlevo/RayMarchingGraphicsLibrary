
use sdl2::{Sdl, video::{GLContext, Window, SwapInterval}, EventPump};

use sdl2::event::Event;
use sdl2::keyboard::Scancode;

mod  input;
use input::*;
mod opengl_shit;
use opengl_shit::*;

pub struct Winsdl {
    pub sdl: Sdl,
    pub window: Window,
    pub gl_context: GLContext,
    pub gl: (),
    pub event_pump: EventPump,
}

impl Winsdl {
    pub fn new(width: usize, height: usize) -> Result<Self, &'static str> {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem
            .window("My window", width as u32, height as u32)
            .opengl()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        let gl = gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        window
            .subsystem()
            .gl_set_swap_interval(SwapInterval::VSync)
            .unwrap();

        let event_pump: sdl2::EventPump = sdl.event_pump().unwrap();

        Ok(Winsdl {
            sdl,
            window,
            gl_context,
            gl,
            event_pump,
        })
    }
}


fn main(){
    let mut win = create_window(500,500);
    let s = win.1;
    let mut win = win.0;
    let mut cam = Camare::new(0.0, 0.0, -3.0,s);
    'main: loop {

        if is_pressed(&win.event_pump,Scancode::W) {
            cam.z += 0.05;
        }
        if is_pressed(&win.event_pump,Scancode::S) {
            cam.z -= 0.05;
        }
        if is_pressed(&win.event_pump,Scancode::A) {
            cam.x += 0.05;
        }
        if is_pressed(&win.event_pump,Scancode::D) {
            cam.x -= 0.05;
        }

        if is_pressed(&win.event_pump,Scancode::Q) {
            cam.angle_x += 0.05;
        }
        if is_pressed(&win.event_pump,Scancode::E) {
            cam.angle_x -= 0.05;
        }

        if is_pressed(&win.event_pump,Scancode::Z) {
            cam.angle_z += 0.05;
        }
        if is_pressed(&win.event_pump,Scancode::X) {
            cam.angle_z -= 0.05;
        }
        
        if is_pressed(&win.event_pump,Scancode::F) {
            cam.angle_y += 0.05;
        }
        if is_pressed(&win.event_pump,Scancode::G) {
            cam.angle_y -= 0.05;
        }
        for event in win.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        cam.sand_info();
        draw(&win,);
    }
}

struct Camare{
    x:f32,
    y:f32,
    z:f32,
    angle_x:f32,
    angle_y:f32,
    angle_z:f32,
    pos_uni:Uniform,
    angle_uni:Uniform,
}

impl Camare {
    fn new(x:f32,y:f32,z:f32,shader:Program)-> Camare{
        let u_pos = Uniform::new(shader.id(), "camarePos").expect("camarePos Uniform");
        let u_angle = Uniform::new(shader.id(), "camareAngles").expect("camreAngles Uniform");
        unsafe {
            gl::Uniform3f(u_angle.id, x,y,z);
            gl::Uniform3f(u_angle.id, 0.0,0.0,0.0);
        }
        Camare{
            x:x,
            y:y,
            z:z,
            angle_x:0.0,
            angle_y:0.0,
            angle_z:0.0,
            pos_uni:u_pos,
            angle_uni:u_angle
        }
    }

    fn sand_info(&self){
        unsafe {
            gl::Uniform3f(self.pos_uni.id, self.x,self.y,self.z);
            gl::Uniform3f(self.angle_uni.id, self.angle_x,self.angle_y,self.angle_z);
        }
    }
    
}

fn create_window(width: usize, height: usize) -> (Winsdl,Program){
    
    let winsdl: Winsdl = Winsdl::new(width, height).unwrap();
    unsafe {gl::Viewport(0, 0, width as i32, height as i32); }

    let program = create_program().unwrap();
    program.set();
    let vertices: Vec<f32> = vec![
        1.0, -1.0,1.0, 0.0, // Bottom right 0
       -1.0,  1.0,0.0, 1.0, // Top left     1
        1.0,  1.0,1.0, 1.0, // Top right    2
       -1.0, -1.0,0.0, 0.0, // Bottom left  3
   ];

    let vbo = Vbo::gen();
    vbo.set(&vertices); 
    let vao = Vao::gen();
    vao.set();  
    let indices: Vec<u32> = vec![
        2, 1, 0, // Top right triangle
        0, 1, 3 // bottom left triangle
    ];

    let ibo = Ibo::gen();
    ibo.set(&indices);
    (winsdl,program)
}

fn draw(win:&Winsdl){
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::DrawElements(
            gl::TRIANGLES, 
            6, 
            gl::UNSIGNED_INT, 
            0 as *const _
        );
        win.window.gl_swap_window();
    }
}

