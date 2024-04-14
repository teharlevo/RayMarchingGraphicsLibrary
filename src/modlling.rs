use sdl2::keyboard::Scancode;
use sdl2::sys::va_list;

use crate::ray_marching_objects::*;
use crate::opengl_shit::*;
use crate::input::*;

pub struct Modlling{
    model_name:String,
    model_code:String,
    model_objects:Vec<ObjForModel>,
    line_x:f32,
    line_y:f32,
    dis:f32,
}

impl Modlling{
    pub fn start(s:&mut Scene) -> Modlling{
        s.clear();
        s.set_shader();
        let cam = &mut s.cam;
        cam.x = 0.0;cam.y = 0.0;cam.z = -10.0;
        cam.angle_x = 0.0;cam.angle_y = 0.0;
        cam.angle_z = 0.0;
        Modlling{
            model_name:String::from("new_object") + &format!("{}",'\n'),
            model_code:String::from("return sdBox( p ,vec3(1.0));"),
            model_objects:vec![],
            line_x:0.0,
            line_y:0.0,
            dis:10.0,
        }
    }

    pub fn update(&mut self,s:&mut Scene,win:&Winsdl){
        if is_pressed(&win.event_pump,Scancode::W) {
            self.line_x += 0.1;
        }
        if is_pressed(&win.event_pump,Scancode::S) {
            self.line_x -= 0.1;
        }
        if is_pressed(&win.event_pump,Scancode::D) {
            self.line_y += 0.1;
        }
        if is_pressed(&win.event_pump,Scancode::A) {
            self.line_y -= 0.1;
        }
        if is_pressed(&win.event_pump,Scancode::E) {
            self.dis += 0.1;
        }
        if is_pressed(&win.event_pump,Scancode::Q) {
            self.dis -= 0.1;
        }
        if is_pressed(&win.event_pump,Scancode::Space){
            s.clear();
            s.add_model(&self.object_text());
            s.set_shader();
            s.add_object(&self.model_name[0..self.model_name.len() - 2]);
        }
        if is_pressed(&win.event_pump,Scancode::Z) {
            let mut x_move = 0.0;
            let mut size = 0.0;
            for i in 0..self.model_objects.len(){
                size = (i as f32 + 1.0);
                x_move += size;
            }
            let obj = ObjForModel{
                type_: ObjForModelType::Box((size,size,size)),
                x: x_move,
                y: 0.0,
                z: 0.0,
                angle: (0.0,0.0,0.0),
            };
            self.model_objects.push(obj);
        }

        let cam = &mut s.cam;

        cam.z = self.line_y.sin() * self.line_x.cos() * -self.dis;
        cam.y = self.line_y.cos() * self.line_x.sin() * -self.dis;
        cam.angle_y = -self.line_x;
        cam.x = self.line_y.sin() * -self.dis;
        cam.z = self.line_x.cos() * self.line_y.cos() * -self.dis;
        cam.angle_x = -self.line_y;
    }

    fn object_text(&mut self) -> String{
        self.update_model_code();
        return format!("{}
        
{}
    {}
{}",self.model_name,"{",self.model_code,"}");
    }

    fn update_model_code(&mut self){
        let mut new_model_code = String::from("");
        let mut i = 0;
        for model in &mut self.model_objects{
            i = i + 1;
            new_model_code = format!("{}
    vec3 q{} = p;
    q{} -= vec3({},{},{});
    q{} = rotateVec3(q{},vec3{:?});
    ",new_model_code,i,i,model.x,model.y,model.z,i,i,model.angle);
            match model.type_ {
                ObjForModelType::Box(pos) => {
                    new_model_code = format!("{}
    float s{} = sdBox(q{},vec3{:?});
    ",new_model_code,i,i,pos);
                },
                ObjForModelType::Sphere(r) => todo!(),
                ObjForModelType::Cylinder(h, r) => todo!(),
                ObjForModelType::Ellipsoid(rx,ry,rz) => todo!(),
                ObjForModelType::Torus(R, r) => todo!(),
                ObjForModelType::Cone(rx, ry, h) => todo!(),
            }
        }
        new_model_code = format!("{}
        return",new_model_code);
        for i in 1..self.model_objects.len(){
            new_model_code = format!("{} opUnion(s{},",new_model_code,i);
        }
        new_model_code = format!("{}s{}",new_model_code,self.model_objects.len());
        for _ in 0..self.model_objects.len() - 1 {
            new_model_code = format!("{})",new_model_code);
        }
        new_model_code = format!("{};",new_model_code);
        if i == 0{
            new_model_code = String::from("return 1000.0;");
        }
        println!("{}",new_model_code);
        self.model_code = new_model_code;
    }
}

enum ObjForModelType {
    Box((f32,f32,f32)),
    Sphere(f32),
    Cylinder(f32,f32),
    Ellipsoid(f32,f32,f32),
    Torus(f32,f32),
    Cone(f32,f32,f32),
}

struct ObjForModel{
    type_:ObjForModelType,
    x:f32,
    y:f32,
    z:f32,
    angle:(f32,f32,f32),
}