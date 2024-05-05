use std:: ffi::CString;

use crate::opengl_shit::*;
use crate::shader_maker::*;


#[derive(Clone)]
pub struct Camare{
    pub x:f32,
    pub y:f32,
    pub z:f32,
    pub dir:(f32,f32,f32),
    pub roll:f32,
}

impl Camare {
    pub fn new(x:f32,y:f32,z:f32)-> Camare{
        
        let cam = Camare{
            x:x,
            y:y,
            z:z,
            dir:(0.0,0.0,1.0),
            roll:0.0,
        };
        cam
    }

    fn send_info(&self,shader:&Program){
        let u_pos = Uniform::new(shader.id(), "camarePos").expect("camarePos Uniform");
        let u_angle = Uniform::new(shader.id(), "camareDir").expect("camreAngles Uniform");
        let u_roll = Uniform::new(shader.id(), "camareRoll").expect("camareRoll Uniform");
        unsafe {
            gl::Uniform3f(u_pos.id, self.x,self.y,self.z);
            gl::Uniform3f(u_angle.id, self.dir.0,self.dir.1,self.dir.2);
            gl::Uniform1f(u_roll.id, self.roll);
        }
    }
    
}

#[derive(Clone)]
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


#[derive(Clone)]
pub struct SceneSttinges{
    pub max_rays:i32,
    pub min_dis_ray:f32,
    pub max_dis_ray:f32,
    pub show_above_min_dis_errors:bool,

    pub color_senstivity:f32,
    pub color_offset:f32,
    pub dis_from_zero:bool,

    pub colors_rgb:[(f32,f32,f32);4],
    pub background:SceneBackGround,
}

impl SceneSttinges {
    pub fn send_info(&self,shader:&Program){
        let u_max_rays = Uniform::new(shader.id(), "maxRays").expect("size Uniform");
        let u_min_dis_rays = Uniform::new(shader.id(), "minDisRay").expect("size Uniform");
        let u_max_dis_rays = Uniform::new(shader.id(), "maxDisRay").expect("size Uniform");
        let u_show_above_min_dis_errors = Uniform::new(shader.id(), "showAboveMinDisErrors").expect("size Uniform");

        let u_color_senstivity = Uniform::new(shader.id(), "colorSenstivity").expect("size Uniform");
        let u_color_offset = Uniform::new(shader.id(), "colorOffset").expect("size Uniform");
        let u_dis_zero = Uniform::new(shader.id(), "disFromZERO").expect("size Uniform");

        let u_color_a = Uniform::new(shader.id(), "colorA").expect("size Uniform");
        let u_color_b = Uniform::new(shader.id(), "colorB").expect("size Uniform");
        let u_color_c = Uniform::new(shader.id(), "colorC").expect("size Uniform");
        let u_color_d = Uniform::new(shader.id(), "colorD").expect("size Uniform");


        unsafe {
            gl::Uniform1i(u_max_rays.id,self.max_rays as i32);
            gl::Uniform1f(u_min_dis_rays.id,self.min_dis_ray as f32);
            gl::Uniform1f(u_max_dis_rays.id,self.max_dis_ray as f32);
            gl::Uniform1i(u_show_above_min_dis_errors.id,self.show_above_min_dis_errors as i32);

            gl::Uniform1f(u_color_senstivity.id,self.color_senstivity as f32);
            gl::Uniform1f(u_color_offset.id,self.color_offset as f32);
            gl::Uniform1i(u_dis_zero.id,self.dis_from_zero as i32);

            gl::Uniform3f(u_color_a.id,self.colors_rgb[0].0,self.colors_rgb[0].1,self.colors_rgb[0].2);
            gl::Uniform3f(u_color_b.id,self.colors_rgb[1].0,self.colors_rgb[1].1,self.colors_rgb[1].2);
            gl::Uniform3f(u_color_c.id,self.colors_rgb[2].0,self.colors_rgb[2].1,self.colors_rgb[2].2);
            gl::Uniform3f(u_color_d.id,self.colors_rgb[3].0,self.colors_rgb[3].1,self.colors_rgb[3].2);


        }
    }
}

#[derive(Clone)]
pub struct Scene{
    pub cam:Camare,
    pub objects:Vec<Object>,
    pub objects_models:Vec<(String,String)>,
    pub sttinges:SceneSttinges,
    scene_width:u32,
    scene_height:u32,
    shader:Program,
}

impl Scene{
    pub fn new(sttinges:SceneSttinges,cam:Camare,scene_width:u32,scene_height:u32)-> Scene{
        let types:Vec<(String, String)> = vec![("".to_string(),"box".to_string())
        ,("".to_string(),"trus".to_string())
        ,("".to_string(),"sphere".to_string())
        ,("".to_string(),"cone".to_string())
        ,("".to_string(),"cylinder".to_string())];

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

    pub fn draw(&self){
        self.shader.bind();
        unsafe { gl::ActiveTexture(gl::TEXTURE0) };

        match &self.sttinges.background {
            //SceneBackGround::SkyBox(cubemap) => {
            //    cubemap.bind();
            //},
            SceneBackGround::Image(image) => {
                image.bind()
            },
            SceneBackGround::Color(r, g,b) => {
                let u_back_ground_color = Uniform::new(self.shader.id(), "backgroundcolor").expect("backgroundcolor Uniform");
                unsafe {
                    gl::Uniform3f(u_back_ground_color.id,*r,*g,*b);
                }
            },
            SceneBackGround::ContinuationOfRay(color_senstivity,color_offset) => {
                let u_color_offset = Uniform::new(self.shader.id(), "ContinuationOfRayColorOffset")
                .expect("ContinuationOfRayColorOffset Uniform");
                let u_color_senstivity = Uniform::new(self.shader.id(), "ContinuationOfRayColorSenstivity")
                .expect("ContinuationOfRayColorSenstivity Uniform");
                unsafe {
                    gl::Uniform1f(u_color_offset.id,*color_offset);
                    gl::Uniform1f(u_color_senstivity.id,*color_senstivity);

                }
            },
            SceneBackGround::FrameBuffer(fra) => {
                fra.get_texture().bind();
            },
        }

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
        draw();

        match &self.sttinges.background {
            //SceneBackGround::SkyBox(cubemap) => {
            //    cubemap.bind();
            //},
            SceneBackGround::Image(image) => {
                image.unbind()
            },
            SceneBackGround::Color(_,_,_) => {
            },
            SceneBackGround::ContinuationOfRay(color_senstivity,color_offset) => {},
            SceneBackGround::FrameBuffer(fra) => {
                fra.get_texture().unbind();
                let u_back_ground_color = Uniform::new(self.shader.id(), "backgroundcolor").expect("backgroundcolor Uniform");
                unsafe {
                    gl::Uniform3f(u_back_ground_color.id,1.0,1.0,0.0);
                }
            },
        }

        //self.shader.unbind();
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

    pub fn update_shader(&mut self){
        self.shader = Scene::shader_maker(&self.objects_models)
    }

    fn shader_maker(ob_types:&Vec<(String,String)>) -> Program{
        let program = create_program(&CString::new(include_str!(".vert")).unwrap()
        ,&make_frag(ob_types)).unwrap();
        
        program
    }

    pub fn get_scene_width(&self) -> u32{
        self.scene_width
    }

    pub fn get_scene_height(&self) -> u32{
        self.scene_height
    }
}

#[derive(Clone)]
pub enum SceneBackGround {
    //SkyBox(Cubemap),
    FrameBuffer(FrameBuffer),
    Image(Texture),
    Color(f32,f32,f32),
    ContinuationOfRay(f32,f32),
}

pub fn draw(){
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::DrawElements(
            gl::TRIANGLES, 
            6, 
            gl::UNSIGNED_INT, 
            0 as *const _
        );
    }
}