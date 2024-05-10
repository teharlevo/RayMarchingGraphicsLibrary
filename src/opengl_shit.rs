use std::{
    ffi::{CStr, CString}, ptr::{null, null_mut}
};

use gl::{
    types::{GLchar, GLenum, GLint, GLuint},
    UseProgram,
};
use gl::TEXTURE_CUBE_MAP;

use image::{EncodableLayout, ImageError};


pub fn create_opengl_contest(width: usize, height: usize){
    
    unsafe {gl::Viewport(0, 0, width as i32, height as i32);}
}

pub fn crate_world_window(){
            

    let vertices: Vec<f32> = vec![
        1.0, -1.0,1.0,0.0,
       -1.0,  1.0,0.0,1.0,
        1.0,  1.0,1.0,1.0,
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
}

/// An OpenGL Shader (of the graphics pipeline)
pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: GLenum) -> Result<Self, String> {
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), null());
            gl::CompileShader(id);
        }

        let mut success: GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            // An error occured
            let mut len: GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(Shader { id })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        //unsafe {
        //    gl::DeleteShader(self.id);
        //}
    }
}

/// An OpenGL Program, a sequence of shaders calls.
#[derive(Clone)]
pub struct Program {
    id: GLuint,
    
}

impl Program {
    fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        let id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(id);
        }

        let mut success: GLint = 1;
        unsafe {
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            // An error occured
            let mut len: GLint = 0;
            unsafe {
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar);
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl::DetachShader(id, shader.id());
            }
        }

        Ok(Program { id:id })
    }

    pub fn id(&self) -> GLuint {
        self.id
    }


    pub fn bind(&self) {
        unsafe {
            UseProgram(self.id);
        }
    }

    pub fn unbind(&self) {
        //unsafe {
        //    UseProgram(0);
        //}
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub fn create_program(vert_string:&CString,frag_string:&CString) -> Result<Program, &'static str> {
    let vert_shader = Shader::from_source(
        vert_string,
        gl::VERTEX_SHADER,
    )
    .unwrap();
    let frag_shader = Shader::from_source(
        frag_string,
        gl::FRAGMENT_SHADER,
    )
    .unwrap();

    let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    Ok(shader_program)
}

/// OpenGL Vertex Buffer Object
pub struct Vbo {
    pub id: GLuint,
}

impl Drop for Vbo {
    fn drop(&mut self) {
    //    self.unbind();
    //    self.delete();
    }
}

impl Vbo {
    pub fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Vbo { id }
    }

    pub fn set(&self, data: &Vec<f32>) {
        self.bind();
        self.data(data);
    }

    fn data(&self, vertices: &Vec<f32>) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

/// OpenGL Index Buffer Object
pub struct Ibo {
    pub id: GLuint,
}

impl Drop for Ibo {
    fn drop(&mut self) {
        //self.unbind();
        //self.delete();
    }
}

impl Ibo {
    pub fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Ibo { id }
    }

    pub fn set(&self, data: &Vec<u32>) {
        self.bind();
        self.data(data);
    }

    fn data(&self, indices: &Vec<u32>) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::DYNAMIC_DRAW,
            );
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

/// OpenGL Vertex Array Object
pub struct Vao {
    pub id: GLuint,
}

impl Drop for Vao {
    fn drop(&mut self) {
        //self.unbind();
        //self.delete();
    }
}

impl Vao {
    pub fn gen() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Vao { id }
    }

    pub fn set(&self) {
        self.bind();
        self.setup();
    }

    fn setup(&self) {
        unsafe {
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (4 * std::mem::size_of::<f32>()) as GLint,
                null(),
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (4 * std::mem::size_of::<f32>()) as GLint,
                (2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

pub struct Uniform {
    pub id: GLint,
}

impl Uniform {
    pub fn new(program: u32, name: &str) -> Result<Self, &'static str> {
        let cname: CString = CString::new(name).expect("CString::new failed");
        let location: GLint = unsafe { gl::GetUniformLocation(program, cname.as_ptr()) };
        if location == -1 {
            return Err("Couldn't get a uniform location");
        }
        Ok(Uniform { id: location })
    }
}

#[derive(Clone)]
pub struct Texture {
    pub id: u32,
}

impl Texture {

    pub fn new(width:u32,height:u32) -> Texture {
        let mut id:u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut id);  
            gl::BindTexture(gl::TEXTURE_2D, id) ;  
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );   
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0) ;  

        }
        
        Texture{
            id
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id) ;  
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn load(&self, path: &str) -> Result<(), ImageError> {
        self.bind();

        let img = image::open(path)?.into_rgba8();
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_bytes().as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        self.unbind();
        
        Ok(())
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, [self.id].as_ptr());
        }
    }
}


//pub struct Cubemap {
//    pub id: u32,
//}
//
//impl Cubemap {
//    pub  fn new() -> Cubemap {
//        let mut id: u32 = 0;
//        unsafe {
//            gl::GenTextures(1, &mut id);     
//        }
//        Cubemap{
//            id
//        }
//    }
//
//    pub fn bind(&self) {
//        unsafe {
//            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.id) ;  
//        }
//    }
//
//    pub fn unbind(&self) {
//        unsafe {
//            gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
//        }
//    }
//
//    pub fn load(&self, path: &str) -> Result<(), ImageError> {
//        unsafe { gl::ActiveTexture(gl::TEXTURE0) };
//        self.unbind();
//        self.bind();
//        let img = image::open(&format!("{}", path))?.into_rgba8();
//        unsafe{
//            let error = gl::GetError();
//            if error != gl::NO_ERROR {
//                println!("OpenGL error: {} + {}", error,"er;y");
//            }
//            let error = gl::GetError();
//            if error != gl::NO_ERROR {
//                println!("OpenGL error: {} + {}", error,"er;y");
//            }
//        }
//            
//        for i in 0..6{
//
//            let bind_point = gl::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32;
//            unsafe {
//                gl::TexImage2D(
//                    bind_point,
//                    0,
//                    gl::RGBA as i32,
//                    img.width() as i32,
//                    img.height() as i32,
//                    0,
//                    gl::RGBA,
//                    gl::UNSIGNED_BYTE,
//                    img.as_bytes().as_ptr() as *const _,
//                );
//                
//            let error = gl::GetError();
//            if error != gl::NO_ERROR {
//                println!("OpenGL error: {} + {}", error,i);
//            }
//            let error = gl::GetError();
//            if error != gl::NO_ERROR {
//                println!("OpenGL error: {} + {}", error,i);
//            }
//            }
//        }
//            unsafe{
//                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
//                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
//                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
//                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
//                gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
//    
//                gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);
//            
//                let error = gl::GetError();
//                if error != gl::NO_ERROR {
//                    println!("OpenGL error: {}", error);
//                }
//            }
//    
//        self.unbind();
//        Ok(())
//    }
//}
//
//impl Drop for Cubemap {
//    fn drop(&mut self) {
//        unsafe {
//            gl::DeleteTextures(1, [self.id].as_ptr());
//        }
//    }
//}

#[derive(Clone)]
pub struct FrameBuffer{
    id:u32,
    tex:Texture,
}

impl FrameBuffer {

    pub fn new(width:u32,height:u32) ->FrameBuffer{

        unsafe {
            let mut id = 0;
            let tex = Texture::new(width, height);
            gl::GenFramebuffers(1, &mut id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, id);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                tex.id,
                0,
            );

            let status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
            if status != gl::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer is not complete!");
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            FrameBuffer { id: id, tex }
        }
    }
    
    pub fn bind(&self){
        unsafe {
             gl::BindFramebuffer(gl::FRAMEBUFFER,self.id);
             gl::Clear(gl::COLOR_BUFFER_BIT);
        };

    }

    pub fn unbind(&self){
        unsafe { gl::BindFramebuffer(gl::FRAMEBUFFER,0);}; 
    }

    pub fn get_id(&self) -> u32{
        self.id
    }

    pub fn get_texture(&self) -> &Texture{
        &self.tex
    }
}