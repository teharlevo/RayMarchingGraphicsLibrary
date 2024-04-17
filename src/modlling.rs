use std::fs::File;
use std::io::Read;
use std::io::Write;

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
        { type_: Box::new(ObjForModelType::Box((3.0,0.5,0.7))),
            displacemen:vec![
                Displacement{
                    type_:Displacement_type::Twist(0.1),
                    afccte_by_trasform:true,
                },
                Displacement{
                    type_:Displacement_type::Repetition((5.0,5.0,5.0),(false,false,false)),
                    afccte_by_trasform:true,
                },
            ],
             x: 0.0,
             y: -0.9,
             z: -0.4,
             angle: (3.14/4.0,3.14/4.0,0.0),
        };
        let modelbox12 = ObjForModel 
        { type_: Box::new(ObjForModelType::Box((0.0,0.0,0.0))),
            displacemen:vec![
                Displacement{
                    type_:Displacement_type::Bend(0.1),
                    afccte_by_trasform:false,
                }
            ],
             x: 0.0,
             y: -0.9,
             z: 0.0,
             angle: (0.0,0.0,0.0),
        };

        let modelbox = ObjForModel 
        { type_: Box::new(ObjForModelType::Union(modelbox1,modelbox12,0.5)),
            displacemen:vec![
                //Displacement::Bend(0.1)
            ],
             x: 0.0,
             y: 0.0,
             z: 0.0,
             angle: (0.0,0.0,0.0),
        };

        let modelbox2 = ObjForModel 
        { type_: Box::new(ObjForModelType::Torus(3.2,0.7)),
            displacemen:vec![],
             x: 0.0,
             y: 0.0,
             z: 0.0,
             angle: (0.0,0.0,0.0),
        };

        let mut modlling = Modlling{
            model_name:String::from("new_object") + &format!("{}",'\n'),
            model_code:String::from("return 1000;"),
            model_objects:ObjForModel 
            { type_: Box::new(ObjForModelType::Union
                (modelbox
                , modelbox2,0.5)),
                displacemen:vec![
                ],
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
        if is_pressed(&win.event_pump,Scancode::R){
            self.export();
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
    vec3 q{i} = {p};
    ");

        new_model_code = Modlling::add_displacements(object,new_model_code,i,false);

        new_model_code = format!("{new_model_code}
    q{i} -= vec3({},{},{});
    q{i} = rotateVec3(q{i},vec3{:?});
    ",object.x,object.y,object.z,object.angle);

        new_model_code = Modlling::add_displacements(object,new_model_code,i,true);

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
            ObjForModelType::Union(ob, ob2,k) => {
                (new_model_code,i) = Modlling::objects_contctor(ob,ob2,&"Union",new_model_code,i,*k);
            },
            ObjForModelType::Subtraction(ob, ob2,k) => {
                (new_model_code,i) = Modlling::objects_contctor(ob,ob2,&"Subtraction",new_model_code,i,*k);
            },
            ObjForModelType::Intersection(ob, ob2,k) => {
                (new_model_code,i) = Modlling::objects_contctor(ob,ob2,&"Intersection",new_model_code,i,*k);
            },
            ObjForModelType::Xor(ob, ob2) => {
                (new_model_code,i) = Modlling::objects_contctor(ob,ob2,&"Xor",new_model_code,i,0.0);
            },
            }
            (new_model_code,i)
    }

    fn objects_contctor(ob:&ObjForModel,ob2:&ObjForModel
        ,name:&str,mut new_model_code:String,mut i:i32,k:f32) -> (String,i32){
        let old_i = i;

        let ob_i = i + 1;
        (new_model_code,i) = Modlling::object_to_model_text(ob,new_model_code,i,old_i);

        let ob_i2 = i + 1;
        (new_model_code,i) = Modlling::object_to_model_text(ob2,new_model_code,i,old_i);

        let name = if k ==0.0 {format!("{}",name)} else{format!("Smooth{}",name)};

        let k: String = if k==0.0{format!("")}else{format!(",{}",k)};

        new_model_code = format!("{}
float s{} = op{}(s{},s{}{});
",new_model_code,old_i,name,ob_i,ob_i2,k);

        (new_model_code,i)
    }

    fn add_displacements(object:&ObjForModel,mut new_model_code:String,i:i32,abt:bool) -> String{
        for displacement in &object.displacemen {
            if abt == displacement.afccte_by_trasform{
                match displacement.type_ {
                    Displacement_type::Twist(k) => {
                        new_model_code = format!("{new_model_code}
        q{i} = opTwist(q{i},{k});
                    ");
                    }
                    Displacement_type::Bend(k) => {
                        new_model_code = format!("{new_model_code}
        q{i} = opCheapBend(q{i},{k});
                    ");
                    }
                    Displacement_type::LimitedRepetition((X,Y,Z), (x,y,z)) => {
                        new_model_code = format!("{new_model_code}
        q{i} = opLimitedRepetition(q{i},vec3({X},{Y},{Z}),vec3({x},{y},{z}));
                    ");
                    }
                    Displacement_type::Repetition((w,h,o),(x,y,z)) => {
                        new_model_code = format!("{new_model_code}
        q{i} = opRepetition(q{i},vec3({w},{h},{o}));
                    ");
                    },
                }
            }
        }
        new_model_code
    }

    fn export(&mut self) -> std::io::Result<()> {
        let file_path = format!("{}.sdf",&self.model_name[0..(self.model_name.len() - 1)]);
        let mut file = File::create(file_path)?;
        file.write_all(self.object_text().as_bytes())?;
        Ok(())
    }
}



enum ObjForModelType {
    Box((f32,f32,f32)),
    Sphere(f32),
    Cylinder(f32,f32),
    Ellipsoid(f32,f32,f32),
    Torus(f32,f32),
    Cone(f32,f32,f32),
    Union(ObjForModel,ObjForModel,f32),
    Subtraction(ObjForModel,ObjForModel,f32),
    Intersection(ObjForModel,ObjForModel,f32),
    Xor(ObjForModel,ObjForModel),
}

enum Displacement_type {
    Twist(f32),
    Bend(f32),
    LimitedRepetition((f32,f32,f32),(f32,f32,f32)),
    Repetition((f32,f32,f32),(bool,bool,bool)),
}

struct Displacement{
    type_:Displacement_type,
    afccte_by_trasform:bool,
}

struct ObjForModel{
    type_:Box<ObjForModelType>,
    displacemen:Vec<Displacement>,
    x:f32,
    y:f32,
    z:f32,
    angle:(f32,f32,f32),
}