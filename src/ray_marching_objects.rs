use std:: ffi::CString;

use sdl2::{ video::{GLContext, SwapInterval, Window}, EventPump, Sdl};
use crate::opengl_shit::*;
use crate::shader_maker::*;


pub struct Winsdl {
    pub sdl: Sdl,
    pub window: Window,
    pub gl_context: GLContext,
    pub gl: (),
    pub event_pump: EventPump,
}

impl Winsdl {
    pub fn new(width: usize, height: usize,title:&str) -> Result<Self, &'static str> {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem
            .window(title, width as u32, height as u32)
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

pub struct Camare{
    pub x:f32,
    pub y:f32,
    pub z:f32,
    pub angle_x:f32,
    pub angle_y:f32,
    pub angle_z:f32,
}

impl Camare {
    pub fn new(x:f32,y:f32,z:f32)-> Camare{
        
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

    fn send_info(&self,shader:&Program){
        let u_pos = Uniform::new(shader.id(), "camarePos").expect("camarePos Uniform");
        let u_angle = Uniform::new(shader.id(), "camareAngles").expect("camreAngles Uniform");
        unsafe {
            gl::Uniform3f(u_pos.id, self.x,self.y,self.z);
            gl::Uniform3f(u_angle.id, self.angle_x,self.angle_y,self.angle_z);
        }
    }
    
}

pub struct Object{
    pub x:f32,
    pub y:f32,
    pub z:f32,
    pub angle_x:f32,
    pub angle_y:f32,
    pub angle_z:f32,
    pub scale:f32,
    pub object_type:i32,
}

impl Object {
    pub fn new(object_type:i32)-> Object{
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

    fn send_info(&self,shader:&Program,num:usize){
        let u_pos = Uniform::new(shader.id(),
         &["tran[",&num.to_string().as_str(),"].pos"].join("")).expect("tran.pos Uniform");
        let u_angle = Uniform::new(shader.id(),
        &["tran[",&num.to_string().as_str(),"].rot"].join("")).expect("tran.rot Uniform");
        let u_scale = Uniform::new(shader.id(), 
        &["tran[",&num.to_string().as_str(),"].scale"].join("")).expect("tran.scale Uniform");
        let u_type = Uniform::new(shader.id(), 
        &["tran[",&num.to_string().as_str(),"].type"].join("")).expect("tran.type Uniform");
        unsafe {
            gl::Uniform3f(u_pos.id, self.x,self.y,self.z);
            gl::Uniform3f(u_angle.id, self.angle_x,self.angle_y,self.angle_z);
            gl::Uniform1f(u_scale.id, self.scale);
            gl::Uniform1i(u_type.id, self.object_type);
        }
    }
}


pub struct SceneSttinges{
    pub max_rays:i32,
    pub min_dis_ray:f32,
    pub max_dis_ray:f32,

    pub color_senstivity:f32,
    pub color_offset:f32,

    pub colors_rgb:[(f32,f32,f32);4],
}

impl SceneSttinges {
    pub fn send_info(&self,shader:&Program){
        let u_max_rays = Uniform::new(shader.id(), "maxRays").expect("size Uniform");
        let u_min_dis_rays = Uniform::new(shader.id(), "minDisRay").expect("size Uniform");
        let u_max_dis_rays = Uniform::new(shader.id(), "maxDisRay").expect("size Uniform");

        let u_color_senstivity = Uniform::new(shader.id(), "colorSenstivity").expect("size Uniform");
        let u_color_offset = Uniform::new(shader.id(), "colorOffset").expect("size Uniform");

        let u_color_a = Uniform::new(shader.id(), "colorA").expect("size Uniform");
        let u_color_b = Uniform::new(shader.id(), "colorB").expect("size Uniform");
        let u_color_c = Uniform::new(shader.id(), "colorC").expect("size Uniform");
        let u_color_d = Uniform::new(shader.id(), "colorD").expect("size Uniform");


        unsafe {
            gl::Uniform1i(u_max_rays.id,self.max_rays as i32);
            gl::Uniform1f(u_min_dis_rays.id,self.min_dis_ray as f32);
            gl::Uniform1f(u_max_dis_rays.id,self.max_dis_ray as f32);

            gl::Uniform1f(u_color_senstivity.id,self.color_senstivity as f32);
            gl::Uniform1f(u_color_offset.id,self.color_offset as f32);

            gl::Uniform3f(u_color_a.id,self.colors_rgb[0].0,self.colors_rgb[0].1,self.colors_rgb[0].2);
            gl::Uniform3f(u_color_b.id,self.colors_rgb[1].0,self.colors_rgb[1].1,self.colors_rgb[1].2);
            gl::Uniform3f(u_color_c.id,self.colors_rgb[2].0,self.colors_rgb[2].1,self.colors_rgb[2].2);
            gl::Uniform3f(u_color_d.id,self.colors_rgb[3].0,self.colors_rgb[3].1,self.colors_rgb[3].2);


        }
    }
}

pub struct Scene{
    pub cam:Camare,
    pub objects:Vec<Object>,
    pub objects_models:Vec<(String,String)>,
    pub sttinges:SceneSttinges,
    scene_width:usize,
    scene_height:usize,
    shader:Program,
}

impl Scene{
    pub fn new(sttinges:SceneSttinges,cam:Camare,scene_width:usize,scene_height:usize)-> Scene{
        let types:Vec<(String, String)> = vec![("".to_string(),"box".to_string())
        ,("".to_string(),"trus".to_string())
        ,("".to_string(),"Sphere".to_string())
        ,("".to_string(),"Cone".to_string())
        ,("".to_string(),"Cylinder".to_string())];

        let shader = Scene::shader_maker(&types);

        let s = Scene{
            cam:cam,
            objects:vec![],
            objects_models:types,
            sttinges:sttinges,
            shader,
            scene_width,
            scene_height,
        };
        s
    }

    pub fn add_object(&mut self,ob_type:&str) -> &mut Object{
        let mut i = 0;
        for (_, name) in &self.objects_models {
            if name == ob_type {
                break;
            }
            i += 1;
        }
        let object = Object::new(  i);
        self.objects.push(object);
        self.objects.last_mut().unwrap()
    }

    pub fn update(&self){
        self.cam.send_info(&self.shader);
        let u_size = Uniform::new(self.shader.id(), "size").expect("size Uniform");
        let u_width= Uniform::new(self.shader.id(), "width").expect("height Uniform");
        let u_height = Uniform::new(self.shader.id(), "height").expect("height Uniform");

        unsafe {
            gl::Uniform1i(u_size.id,self.objects.len() as i32);
            gl::Uniform1f(u_width.id ,self.scene_width  as f32);
            gl::Uniform1f(u_height.id,self.scene_height as f32);
        }
        for i in 0..self.objects.len(){
            self.objects[i].send_info(&self.shader, i);
        }
        self.sttinges.send_info(&self.shader);
    }

    pub fn add_folder_to_model(&mut self,folder_path: &str){
        let objects =  get_dis_funcans_folder(folder_path);
        if Some(&objects).is_some(){
            let objects= objects.unwrap();
            for object in objects{
                self.objects_models.push(object);
            }

        }
    }
    
    pub fn add_model(&mut self,model_texts: &str){
        let object =  add_object(model_texts);
        self.objects_models.push(object);
    }

    pub fn clear_objects_models(&mut self){
        self.objects_models = vec![("".to_string(),"box".to_string())
        ,("".to_string(),"trus".to_string())
        ,("".to_string(),"Sphere".to_string())
        ,("".to_string(),"Cone".to_string())
        ,("".to_string(),"Cylinder".to_string())];
    }

    pub fn clear_objects(&mut self){
        self.objects = vec![];
    }

    pub fn clear(&mut self){
        self.clear_objects();
        self.clear_objects_models();
    }

    pub fn shader_text(&mut self) -> String{
        format!("{}
        {}",&CString::new(include_str!(".vert")).unwrap().to_string_lossy().into_owned()
        ,&make_frag(&self.objects_models).to_string_lossy().into_owned())
    }

    pub fn set_shader(&mut self){
        self.shader = Scene::shader_maker(&self.objects_models)
    }

    fn shader_maker(ob_types:&Vec<(String,String)>) -> Program{
        let program = create_program(&CString::new(include_str!(".vert")).unwrap()
        ,&make_frag(ob_types)).unwrap();
        
        program.set();
        program
    }
}

pub fn create_window(width: usize, height: usize,title:&str) -> Winsdl{
    
    let winsdl: Winsdl = Winsdl::new(width, height,title).unwrap();
    unsafe {gl::Viewport(0, 0, width as i32, height as i32); }
    let vertices: Vec<f32> = vec![
        1.0, -1.0,1.0,0.0,
       -1.0,  1.0,0.0,1.0,
        1.0,  1.0,1.0, 1.0,
       -1.0, -1.0,0.0,0.0,
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

pub fn draw(win:&Winsdl){
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