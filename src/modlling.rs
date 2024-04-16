use sdl2::keyboard::Scancode;
use sdl2::sys::va_list;

use crate::ray_marching_objects::*;
use crate::opengl_shit::*;
use crate::input::*;

pub struct Modlling{
    model_name:String,
    model_code:String,
    model_objects:ObjForModel,
    line_x:f32,
    line_y:f32,
    dis:f32,
}

impl Modlling{
    pub fn start(s:&mut Scene) -> Modlling{
        let cam = &mut s.cam;
        cam.x = 0.0;cam.y = 0.0;cam.z = -10.0;
        cam.angle_x = 0.0;cam.angle_y = 0.0;
        cam.angle_z = 0.0;

        let modelbox1 = ObjForModel 
        { type_: Box::new(ObjForModelType::Box((1.5,0.5,1.0))),
             x: 0.0,
             y: 0.0,
             z: 0.0,
             angle: (0.0,0.0,0.0),
        };
        let modelbox2 = ObjForModel 
        { type_: Box::new(ObjForModelType::Torus(1.2,0.7)),
             x: 0.0,
             y: 0.0,
             z: 0.0,
             angle: (0.0,0.0,0.0),
        };

        let mut modlling = Modlling{
            model_name:String::from("new_object") + &format!("{}",'\n'),
            model_code:String::from("return 1000;"),
            model_objects:ObjForModel 
            { type_: Box::new(ObjForModelType::Xor
                (modelbox1
                , modelbox2)),
                 x: 0.0,
                 y: 0.0,
                 z: 0.0,
                 angle: (0.0,0.0,0.0) },
            line_x:0.0,
            line_y:3.14/2.0,
            dis:10.0,
        };
        modlling.reset(s);
        modlling
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
            self.dis += 0.3;
        }
        if is_pressed(&win.event_pump,Scancode::Q) {
            self.dis -= 0.3;
        }
        if is_pressed(&win.event_pump,Scancode::Space){
            self.reset(s);
        }

        let cam = &mut s.cam;


        let obj = s.objects.first_mut().unwrap();
        obj.angle_x = self.line_x;  
        obj.angle_z = self.line_y;
        cam.z = -self.dis;
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
        (new_model_code,i) = Self::object_to_model_text(&mut self.model_objects,new_model_code, i,0);
        new_model_code = format!("{}
        return s{};",new_model_code,1);
        println!("{}",new_model_code);
        if i == 0{
            new_model_code = String::from("return 1000.0;");
            self.model_code = new_model_code;
            return;
        }
        
        self.model_code = new_model_code;
    }

    fn reset(&mut self,s:&mut Scene){

        s.clear();
        s.add_model(&self.object_text());
        s.set_shader();
        s.add_object(&self.model_name[0..self.model_name.len() - 2]);

    }

    fn object_to_model_text(object:&ObjForModel,mut new_model_code:String,mut i:i32,origen:i32) -> (String,i32){
        i += 1;
        let p = if origen == 0{String::from("p")}else{format!("q{}",origen)};
        new_model_code = format!("{new_model_code}
    vec3 q{} = {p};
    q{} -= vec3({},{},{});
    q{} = rotateVec3(q{},vec3{:?});
    ",i,i,object.x,object.y,object.z,i,i,object.angle);

            let type_ = object.type_.as_ref();
            match type_ {
                ObjForModelType::Box(pos) => {
                    new_model_code = format!("{}
    float s{} = sdBox(q{},vec3{:?});
    ",new_model_code,i,i,pos);
                },

                ObjForModelType::Sphere(r) => {
                    new_model_code = format!("{}
    float s{} = sdSphere(q{},{});
    ",new_model_code,i,i,r);
                },

                ObjForModelType::Cylinder(h, r) =>{
                    new_model_code = format!("{}
    float s{} = sdCylinder(q{},{},{});
    ",new_model_code,i,i,h, r);
                },

                ObjForModelType::Ellipsoid(rx,ry,rz) => {
                    new_model_code = format!("{}
    float s{} = sdEllipsoid(q{},vec3({},{},{}));
    ",new_model_code,i,i,rx,ry,rz);
                },
                ObjForModelType::Torus(R, r) => {
                    new_model_code = format!("{}
    float s{} = sdTorus(q{},vec2({},{}));
    ",new_model_code,i,i,R,r);
                },

                ObjForModelType::Cone(rx, ry, h) => {
                    new_model_code = format!("{}
    float s{} = sdBox(q{},vec2({},{}),{});
    ",new_model_code,i,i,rx,ry,h);
                },
                ObjForModelType::Union(ob, ob2) => {
                    (new_model_code,i) = Modlling::k(ob,ob2,&"Union",new_model_code,i);
                },
                ObjForModelType::Subtraction(ob, ob2) => {
                    (new_model_code,i) = Modlling::k(ob,ob2,&"Subtraction",new_model_code,i);
                },
                ObjForModelType::Intersection(ob, ob2) => {
                    (new_model_code,i) = Modlling::k(ob,ob2,&"Intersection",new_model_code,i);
                },
                ObjForModelType::Xor(ob, ob2) => {
                    (new_model_code,i) = Modlling::k(ob,ob2,&"Xor",new_model_code,i);
                },
            }
            (new_model_code,i)
    }

    fn k(ob:&ObjForModel,ob2:&ObjForModel
        ,name:&str,mut new_model_code:String,mut i:i32) -> (String,i32){
        let old_i = i;
        (new_model_code,i) = Modlling::object_to_model_text(ob,new_model_code,i,old_i);
        let ob_i = i;
        (new_model_code,i) = Modlling::object_to_model_text(ob2,new_model_code,i,old_i);
        let ob_i2 = i;
        new_model_code = format!("{}
float s{} = op{}(s{},s{});
",new_model_code,old_i,name,ob_i,ob_i2);
        (new_model_code,i)
    }
}



enum ObjForModelType {
    Box((f32,f32,f32)),
    Sphere(f32),
    Cylinder(f32,f32),
    Ellipsoid(f32,f32,f32),
    Torus(f32,f32),
    Cone(f32,f32,f32),
    Union(ObjForModel,ObjForModel),
    Subtraction(ObjForModel,ObjForModel),
    Intersection(ObjForModel,ObjForModel),
    Xor(ObjForModel,ObjForModel),
}

struct ObjForModel{
    type_:Box<ObjForModelType>,
    x:f32,
    y:f32,
    z:f32,
    angle:(f32,f32,f32),
}