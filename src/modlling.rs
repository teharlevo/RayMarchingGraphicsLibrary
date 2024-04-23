
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use sdl2::keyboard::Scancode;
use sdl2::sys::va_list;

use crate::ray_marching_objects::*;
use crate::opengl_shit::*;
use crate::input::*;

use crate::sdl2objects::*;

pub struct Modlling{
    model_name:String,
    model_code:String,
    model_object:ObjForModel,
    line_x:f32,
    line_y:f32,
    dis:f32,
    update_lest_frame:bool,
    exported_lest_frame:bool,
}

impl Modlling{
    pub fn start(s:&mut Scene) -> Modlling{
        let cam = &mut s.cam;
        cam.x = 0.0;cam.y = 0.0;cam.z = -10.0;
        cam.angle_x = 0.0;cam.angle_y = 0.0;
        cam.angle_z = 0.0;

        let mut modlling = Modlling{
            model_name:String::from("new_object") + &format!("{}",'\n'),
            model_code:String::from("return 1000;"),
            model_object:ObjForModel{
                type_: Box::new(ObjForModelType::Empty()),
                displacemen:vec![],
                x: 0.0,
                y: 0.0,
                z: 0.0,
                angle: (0.0,0.0,0.0),
            },

            line_x:0.0,
            line_y:3.14/2.0,
            dis:10.0,
            update_lest_frame:   false,
            exported_lest_frame: false,
        };
        modlling.reset(s);
        modlling
    }

    pub fn update(&mut self,s:&mut Scene,win:&Winsdl){
        let speed = if is_pressed(&win.event_pump,Scancode::LShift) {5.0}else{1.0};

        if is_pressed(&win.event_pump,Scancode::W) {
            self.line_x += 0.1 * speed;
        }
        if is_pressed(&win.event_pump,Scancode::S) {
            self.line_x -= 0.1 * speed;
        }
        if is_pressed(&win.event_pump,Scancode::D) {
            self.line_y += 0.1 * speed;
        }
        if is_pressed(&win.event_pump,Scancode::A) {
            self.line_y -= 0.1 * speed;
        }
        if is_pressed(&win.event_pump,Scancode::E) {
            self.dis += 0.3 * speed;
        }
        if is_pressed(&win.event_pump,Scancode::Q) {
            self.dis -= 0.3 * speed;
        }
        if is_pressed(&win.event_pump,Scancode::Q) {
            self.dis -= 0.3 * speed;
        }
        if is_pressed(&win.event_pump,Scancode::F) && is_pressed(&win.event_pump,Scancode::LCtrl) {
            self.dis = 0.0;
            self.line_x = 0.0;
            self.line_y = 0.0;
        }

        if is_pressed(&win.event_pump,Scancode::Space) && !self.update_lest_frame{
            self.reset(s);
            self.update_lest_frame = true;
        }
        else if !is_pressed(&win.event_pump,Scancode::Space) {
            self.update_lest_frame = false;
        }
        if is_pressed(&win.event_pump,Scancode::R) && is_pressed(&win.event_pump,Scancode::LCtrl)
        && !self.exported_lest_frame{
            self.export();
            self.exported_lest_frame = true;
        }
        else if !is_pressed(&win.event_pump,Scancode::R) || !is_pressed(&win.event_pump,Scancode::LCtrl) {
            self.exported_lest_frame = false;
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
        (new_model_code,i) = Self::object_to_model_text(&mut self.model_object,new_model_code, i,0);
        new_model_code = format!("{}
        return s{};",new_model_code,1);
        if i == 0{
            new_model_code = String::from("return 1000.0;");
            self.model_code = new_model_code;
            return;
        }
        
        self.model_code = new_model_code;
    }

    fn reset(&mut self,s:&mut Scene){
        let new_object = make_object_sdf_maker();
        match new_object {
            Ok((ob,sttings,ob_name)) => {
                self.model_object = ob;
                s.sttinges = sttings;
                self.model_name = ob_name;},
            Err(text) => println!("{text}"),
        }
        s.clear();
        s.add_model(&self.object_text());
        s.set_shader();
        s.add_object(&self.model_name);
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
            
            ObjForModelType::Cone(angle, h) => {
                new_model_code = format!("{}
    float s{} = sdCone(q{},vec2({},{}),{});
    ",new_model_code,i,i,angle.sin(),angle.cos(),h);
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
            ObjForModelType::Empty() => {
                new_model_code = format!("{}
                float s{} = maxDisRay * 2.0;
                ",new_model_code,i); 
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
            if abt == displacement.afcctet_by_trasform{
                match displacement.type_ {
                    DisplacementType::Twist(k) => {
                        new_model_code = format!("{new_model_code}
        q{i} = opTwist(q{i},{k});
                    ");
                    }
                    DisplacementType::Bend(k) => {
                        new_model_code = format!("{new_model_code}
        q{i} = opCheapBend(q{i},{k});
                    ");
                    }
                    DisplacementType::LimitedRepetition((X,Y,Z), (x,y,z)) => {
                        new_model_code = format!("{new_model_code}
        q{i} = opLimitedRepetition(q{i},vec3({X},{Y},{Z}),vec3({x},{y},{z}));
                    ");
                    }

                    DisplacementType::Repetition((w,h,o),(x,y,z)) => {
                    if x || y | z{
                        let kotert = format!("{}{}{}",
                        if x{"X"}else{""},if y{"Y"}else{""},if z{"Z"}else{""});

                        new_model_code = format!("{new_model_code}
        q{i} = opRepetition{kotert}(q{i},vec3({w},{h},{o}));
                    ");
                        }
                    },
                }
            }
        }
        new_model_code
    }

    fn export(&mut self) -> std::io::Result<()> {
        let file_path = format!("{}.sdf",&self.model_name);
        let mut file = File::create(file_path)?;
        file.write_all(self.object_text().as_bytes())?;
        Ok(())
    }

}

enum ObjForModelType {
    Empty(),
    Box((f32,f32,f32)),
    Sphere(f32),
    Cylinder(f32,f32),
    Ellipsoid(f32,f32,f32),
    Torus(f32,f32),
    Cone(f32,f32),
    Union(ObjForModel,ObjForModel,f32),
    Subtraction(ObjForModel,ObjForModel,f32),
    Intersection(ObjForModel,ObjForModel,f32),
    Xor(ObjForModel,ObjForModel),
}

enum DisplacementType {
    Twist(f32),
    Bend(f32),
    LimitedRepetition((f32,f32,f32),(f32,f32,f32)),
    Repetition((f32,f32,f32),(bool,bool,bool)),
}

struct Displacement {
    type_:DisplacementType,
    afcctet_by_trasform:bool,
}

struct ObjForModel{
    type_:Box<ObjForModelType>,
    displacemen:Vec<Displacement>,
    x:f32,
    y:f32,
    z:f32,
    angle:(f32,f32,f32),
}

enum Word {
    num(f32),
    word(String),
}

fn make_object_sdf_maker() -> Result<(ObjForModel,SceneSttinges,String),String>{
    if !Path::new("object_maker.sdfMaker").exists(){
        let file = File::create("object_maker.sdfMaker");
        match file{
            Ok(mut file_to_write) => {let _ = file_to_write.write(b"
name:new_object
max_rays: 60,
min_dis_ray: 0.1,
max_dis_ray: 500.0,
color_senstivity: 0.1,
color_offset:0.0,

colors[(0.5, 0.5, 0.5),(0.5, 0.5, 0.5),(1.0, 1.0, 1.0),(0.00, 0.33, 0.67)]            
empty
pos(0.0,0.0,0.0)
rot(0.0,0.0,0.0)
");},

            Err(_) => return Err(String::from("there are not object_maker.sdfMaker file field to make one")),
        }
    }

    let file_read = fs::read_to_string("object_maker.sdfMaker");
    let file_read = 
    match file_read{
        Ok(file_to_write) => {file_to_write},
        Err(_) => return Err(String::from("field to read object_maker.sdfMaker")),
    };

    let mut word_list = text_to_word_list(&file_read);
    let mut name = String::from("new_object");
    loop{

        if word_list.len() <= 2{
            return Err(String::from("sertch for object not that not exsit fix!"));
        }
        let word = word_list.remove(0);

        match word {
            Word::num(_) => {},
            Word::word(str) => {
                if str == "name"{
                    break;
                }
            },
        }
    }    let word = word_list.remove(0);
    match word {
        Word::num(g) => {
            name = format!("{g}");
        },
        Word::word(str) => {
            name = str;
        },
    }

    let settings =   SceneSttinges{
        max_rays: num_in_word_list(&mut word_list)? as i32,
        min_dis_ray: num_in_word_list(&mut word_list)?,
        max_dis_ray: num_in_word_list(&mut word_list)?,
        color_senstivity: num_in_word_list(&mut word_list)?,
        color_offset: num_in_word_list(&mut word_list)?,
        colors_rgb: [(num_in_word_list(&mut word_list)?,num_in_word_list(&mut word_list)?,num_in_word_list(&mut word_list)?)
        ,(num_in_word_list(&mut word_list)?,num_in_word_list(&mut word_list)?,num_in_word_list(&mut word_list)?)
        ,(num_in_word_list(&mut word_list)?,num_in_word_list(&mut word_list)?,num_in_word_list(&mut word_list)?)
        ,(num_in_word_list(&mut word_list)?,num_in_word_list(&mut word_list)?,num_in_word_list(&mut word_list)?)],
    };

    Ok((object_maker_from_word_list(&mut word_list)?,settings,name))
}

fn object_maker_from_word_list(word_list:&mut Vec<Word>) -> Result<ObjForModel,String>{
    if word_list.len() == 0{
        return Err(String::from("sertch for object not that not exsit fix!"));
    }

    let mut serch_for_object = true;
    let mut object_type = ObjForModelType::Empty();

    while serch_for_object {

        let word = word_list.remove(0);

        if word_list.len() == 0{
            return Err(String::from("sertch for object not that not exsit fix!"));
        }

        match word{
            Word::num(_) => {},
            Word::word(str) => {
                if str == "empty" {
                    object_type = ObjForModelType::Empty();
                    serch_for_object = false;
                }
                else if str == "box" {
                    object_type = ObjForModelType::Box(
                    (num_in_word_list(word_list)?,num_in_word_list(word_list)?,num_in_word_list(word_list)?));
                    serch_for_object = false;
                }
                else if str == "sphere" {
                    object_type = ObjForModelType::Sphere(
                    num_in_word_list(word_list)?);
                    serch_for_object = false;
                }
                else if str == "cylinder" {
                    object_type = ObjForModelType::Cylinder(
                    num_in_word_list(word_list)?,num_in_word_list(word_list)?);
                    serch_for_object = false;
                }
                else if str == "ellipsoid" {
                    object_type = ObjForModelType::Ellipsoid(
                    num_in_word_list(word_list)?,num_in_word_list(word_list)?,num_in_word_list(word_list)?);
                    serch_for_object = false;
                }
                else if str == "torus"{
                    object_type = ObjForModelType::Torus(
                        num_in_word_list(word_list)?,num_in_word_list(word_list)?);
                        serch_for_object = false;
                }
                else if str == "cone"{
                    object_type = ObjForModelType::Cone(
                        num_in_word_list(word_list)? / (180.0/3.1415),num_in_word_list(word_list)?);
                        serch_for_object = false;
                }
                else if str == "union"{
                    object_type = ObjForModelType::Union(
                        object_maker_from_word_list(word_list)?,object_maker_from_word_list(word_list)?,num_in_word_list(word_list)?
                    );
                    serch_for_object = false;
                }
                else if str == "subtraction" || str == "sub"{
                    object_type = ObjForModelType::Subtraction(
                        object_maker_from_word_list(word_list)?,object_maker_from_word_list(word_list)?,num_in_word_list(word_list)?
                    );
                    serch_for_object = false;
                }
                else if str == "intersection" || str == "inter"{
                    object_type = ObjForModelType::Intersection(
                        object_maker_from_word_list(word_list)?,object_maker_from_word_list(word_list)?,num_in_word_list(word_list)?
                    );
                    serch_for_object = false;
                }
                else if str == "xor" || str == "Xor" {
                    object_type = ObjForModelType::Xor(
                        object_maker_from_word_list(word_list)?,object_maker_from_word_list(word_list)?);
                    serch_for_object = false;
                }
            },
        }
    }
    Ok(
        ObjForModel { type_: Box::new(object_type), 
            displacemen: displacement_list_maker_from_word_list(word_list)?,
            x: num_in_word_list(word_list)?,
            y: num_in_word_list(word_list)?,
            z: num_in_word_list(word_list)?,
            angle: (num_in_word_list(word_list)? / (180.0/3.1415)
            ,num_in_word_list(word_list)? / (180.0/3.1415)
            ,num_in_word_list(word_list)? / (180.0/3.1415)) 
        }
    )
}

fn displacement_list_maker_from_word_list(word_list:&mut Vec<Word>) ->  Result<Vec<Displacement>,String>{
    let mut list = vec![];
    let mut serch_for_end = true;

    while serch_for_end {
        
        if word_list.len() == 0{
            return Err(String::from("prablem in displacement list"));
        }
        
        let word = word_list.remove(0);

        match word{
            Word::num(_) => {},
            Word::word(str) => {
                if str == "end" || str == "End" || str == "END" {
                    println!("lol");
                    serch_for_end = false;
                }
                else if str == "Twist" || str == "twist" {
                    list.push(Displacement { 
                        type_:DisplacementType::Twist(num_in_word_list(word_list)?), 
                        afcctet_by_trasform:bool_in_word_list(word_list)?,
                    })
                }
                else if str == "Bend" || str == "bend" {
                    list.push(Displacement { 
                        type_:DisplacementType::Bend(num_in_word_list(word_list)?), 
                        afcctet_by_trasform:bool_in_word_list(word_list)?,
                    })
                }
                else if str == "LimitedRepetition" || str == "Limitedrepetition" || str == "LimitedRepetition"
                || str == "limrep" {
                    list.push(Displacement { 
                        type_:DisplacementType::LimitedRepetition((num_in_word_list(word_list)?,num_in_word_list(word_list)?,num_in_word_list(word_list)?)
                    ,(num_in_word_list(word_list)?,num_in_word_list(word_list)?,num_in_word_list(word_list)?)), 
                        afcctet_by_trasform:bool_in_word_list(word_list)?,
                    })
                }
                else if str == "Repetition" || str == "repetition" || str == "rep"{
                    list.push(Displacement { 
                        type_:DisplacementType::Repetition((num_in_word_list(word_list)?,num_in_word_list(word_list)?,num_in_word_list(word_list)?)
                    ,(bool_in_word_list(word_list)?,bool_in_word_list(word_list)?,bool_in_word_list(word_list)?)), 
                        afcctet_by_trasform:bool_in_word_list(word_list)?,
                    })
                }
            },
        }

        //Twist(f32),
        //Bend(f32),
        //LimitedRepetition((f32,f32,f32),(f32,f32,f32
        //Repetition((f32,f32,f32),(bool,bool,bool)),
    }
    Ok(list)
}

fn text_to_word_list(text:&str) -> Vec<Word>{

    let mut word_list = vec![];
    let lines = text.lines();
    for line in lines{
        let mut m = 0;
        let line = line.replace(",", " ");
        let line = line.replace(":", " ");
        let line = line.replace(";", " ");
        let line = line.replace(")", " ");
        let line = line.replace("(", " ");

        let words = line.split(" ");
        for word in words{
            m = m + 1;
            match word.parse::<f32>() {
                Ok(num) =>{
                    word_list.push(Word::num(num));
                },
                Err(_) => {
                    word_list.push(Word::word(String::from(word)));
                },
            }
        }
    }
    word_list
}

fn num_in_word_list(word_list:&mut Vec<Word>) -> Result<f32,String>{
    if word_list.len() == 0{
        return Err(String::from("sertch for num not that not exsit fix!"));
    }
    loop {
        let word = word_list.remove(0);
        if word_list.len() == 0{
            return Err(String::from("sertch for num not that not exsit fix!"));
        }
        match word {
            Word::num(num) => {return Ok(num)},
            Word::word(_) => {},
        }
    }
}

fn bool_in_word_list(word_list:&mut Vec<Word>) -> Result<bool,String>{
    if word_list.len() == 0{
        return Err(String::from("sertch for bool not that not exsit fix!"));
    }
    loop {
        let word = word_list.remove(0);
        if word_list.len() == 0{
            return Err(String::from("sertch for bool not that not exsit fix!"));
        }
        match word {
            Word::num(_) => {},
            Word::word(w) => {
                if w == "true" || w == "True" || w == "T" || w == "t"{
                    return Ok(true);
                }
                else if w == "false" || w == "False" || w == "f" || w == "F"{
                    return Ok(false);
                }
            },
        }
    }
}