
use std::{any::Any, ffi::CString};

use sdl2::{sys::SDL_SetWindowSize, video::{GLContext, SwapInterval, Window}, EventPump, Sdl};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Scancode;

mod  input;
use input::*;
mod opengl_shit;
use opengl_shit::*;
mod shader_maker;
use shader_maker::*;

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
    let cam = Camare::new(0.0, 0.0, -3.0);
    let mut se = Scene::new(cam);
    se.add_folder_to_objects("src/objects/");
    let win = create_window(500,500,&se);
    let mut win = win;

    let mut s = se.shader_ajjster(); 

    let box1 = se.add_object("BoxFrame");
    box1.angle_z = 3.14/4.0;box1.angle_y = 3.14/4.0;
    let box1 = se.add_object("coolspere");
    //for i in 0..30{
    //    let box1 = se.add_object("trus") ;
    //    box1.z = box1.z + i as f32 * 2.0 - 30.0;
    //    box1.angle_x = 3.14/2.0;box1.scale = 1.3;
    //}
    let mut time = Instant::now();
    let mut fps = 0;

    'main: loop {

        if is_pressed(&win.event_pump,Scancode::W) {
            se.cam.z += 0.05;
        }
        if is_pressed(&win.event_pump,Scancode::S) {
            se.cam.z -= 0.05;
            //let program = create_program().unwrap();
            //program.set();
        }
        if is_pressed(&win.event_pump,Scancode::A) {
            se.cam.x -= 0.05;
        }
        if is_pressed(&win.event_pump,Scancode::D) {
            se.cam.x += 0.05;
        }

        if is_pressed(&win.event_pump,Scancode::Q) {
            se.cam.angle_x += 0.05;
        }
        if is_pressed(&win.event_pump,Scancode::E) {
            se.cam.angle_x -= 0.05;
        }

        if is_pressed(&win.event_pump,Scancode::Z) {
            se.cam.angle_z += 0.05;
        }
        if is_pressed(&win.event_pump,Scancode::X) {
            se.cam.angle_z -= 0.05;
        }
        
        if is_pressed(&win.event_pump,Scancode::F) {
            se.cam.angle_y += 0.05;
        }
        if is_pressed(&win.event_pump,Scancode::G) {
            se.cam.angle_y -= 0.05;
            //unsafe { 
            //    SDL_SetWindowSize(win.window.raw(),500,700);
            //}
        }
        //box1.angle_z += 1.0;
        if is_pressed(&win.event_pump,Scancode::Space) {
            se.clear_objects();
            se.add_folder_to_objects("src/objects/");
            s = se.shader_ajjster();
        }
        for event in win.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        draw(&win,);
        se.sand_info(&s);

        if Instant::now().duration_since(time).as_secs_f32() > 1.0 {
            println!("fps:{}", fps);
            fps = 0;
            time = Instant::now();
        }
        fps = fps + 1;
    }
}

struct Camare{
    x:f32,
    y:f32,
    z:f32,
    angle_x:f32,
    angle_y:f32,
    angle_z:f32,
}

impl Camare {
    fn new(x:f32,y:f32,z:f32)-> Camare{
        
        let cam = Camare{
            x:x,
            y:y,
            z:z,
            angle_x:0.0,
            angle_y:0.0,
            angle_z:0.0,
        };
        cam
    }

    fn sand_info(&self,shader:&Program){
        let u_pos = Uniform::new(shader.id(), "camarePos").expect("camarePos Uniform");
        let u_angle = Uniform::new(shader.id(), "camareAngles").expect("camreAngles Uniform");
        unsafe {
            gl::Uniform3f(u_pos.id, self.x,self.y,self.z);
            gl::Uniform3f(u_angle.id, self.angle_x,self.angle_y,self.angle_z);
        }
    }
    
}

struct Object{
    x:f32,
    y:f32,
    z:f32,
    angle_x:f32,
    angle_y:f32,
    angle_z:f32,
    scale:f32,
    object_type:i32,
}

impl Object {
    fn new(object_type:i32)-> Object{
        let ob = Object{
            x:0.0,
            y:0.0,
            z:0.0,
            angle_x:0.0,
            angle_y:0.0,
            angle_z:0.0,
            scale:1.0,
            object_type:object_type,
        };
        return ob;
    }

    fn sand_info(&self,shader:&Program,num:usize){
        let u_pos = Uniform::new(shader.id(),
         &["tran[",&num.to_string().as_str(),"].pos"].join("")).expect("tran.pos Uniform");
        let u_angle = Uniform::new(shader.id(),
        &["tran[",&num.to_string().as_str(),"].rot"].join("")).expect("tran.rot Uniform");
        let u_scale = Uniform::new(shader.id(), 
        &["tran[",&num.to_string().as_str(),"].scale"].join("")).expect("tran.scale Uniform");
        let u_type = Uniform::new(shader.id(), 
        &["tran[",&num.to_string().as_str(),"].type"].join("")).expect("tran.scale Uniform");
        unsafe {
            gl::Uniform3f(u_pos.id, self.x,self.y,self.z);
            gl::Uniform3f(u_angle.id, self.angle_x,self.angle_y,self.angle_z);
            gl::Uniform1f(u_scale.id, self.scale);
            gl::Uniform1i(u_type.id, self.object_type);
        }
    }
}
struct Scene{
    cam:Camare,
    objects:Vec<Object>,
    objects_models:Vec<(String,String)>
}

impl Scene{
    fn new(cam:Camare)-> Scene{
        let types:Vec<(String, String)> = vec![("".to_string(),"box".to_string())
        ,("".to_string(),"trus".to_string())
        ,("".to_string(),"Sphere".to_string())
        ,("".to_string(),"Cone".to_string())
        ,("".to_string(),"Cylinder".to_string())];

        let s = Scene{
            cam:cam,
            objects:vec![],
            objects_models:types,
        };
        s
    }

    fn add_object(&mut self,ob_type:&str) -> &mut Object{
        let mut i = 0;
        for (code, name) in &self.objects_models {
            if name == ob_type {
                break;
            }
            i += 1;
        }
        let object = Object::new(  i);
        self.objects.push(object);
        self.objects.last_mut().unwrap()
    }

    fn sand_info(&self,shader:&Program){
        self.cam.sand_info(shader);
        let u_size = Uniform::new(shader.id(), "size").expect("size Uniform");
        unsafe {
            gl::Uniform1i(u_size.id,self.objects.len() as i32);
        }
        for i in 0..self.objects.len(){
            self.objects[i].sand_info(shader, i);
        }
    }

    fn add_folder_to_objects(&mut self,folder_path: &str){
        let objects =  get_dis_funcans_folder(folder_path);
        if Some(&objects).is_some(){
            let objects= objects.unwrap();
            for object in objects{
                self.objects_models.push(object);
            }

        }
    }
    fn clear_objects(&mut self){
        self.objects_models = vec![("".to_string(),"box".to_string())
        ,("".to_string(),"trus".to_string())
        ,("".to_string(),"Sphere".to_string())
        ,("".to_string(),"Cone".to_string())
        ,("".to_string(),"Cylinder".to_string())];
    }

    fn shader_ajjster(&self) -> Program{
        let program = create_program(&CString::new(include_str!(".vert")).unwrap()
        ,&make_frag(&self.objects_models)).unwrap();
        
        
        program.set();
        program
    }
}

fn create_window(width: usize, height: usize,s:&Scene) -> Winsdl{
    
    let winsdl: Winsdl = Winsdl::new(width, height).unwrap();
    unsafe {gl::Viewport(0, 0, width as i32, height as i32); }

    let aspect = width as f32/height as f32;
    let vertices: Vec<f32> = vec![
        1.0, -1.0,aspect,0.0, // Bottom right 0
       -1.0,  1.0,0.0,1.0, // Top left     1
        1.0,  1.0,aspect, 1.0, // Top right    2
       -1.0, -1.0,0.0,0.0, // Bottom left  3
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
    winsdl
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

