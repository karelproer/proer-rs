extern crate nalgebra;

pub struct ShaderProgram {
    id: u32,
}

impl ShaderProgram {
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl super::super::shader::Shader for ShaderProgram {
    fn new(vertex_source: &str, fragment_source: &str) -> Option<Self> {
        unsafe {
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            assert_ne!(vertex, 0);
            gl::ShaderSource(vertex, 1, &(vertex_source.as_ptr().cast()) as *const *const i8, &(vertex_source.len() as i32) as *const i32);
            gl::CompileShader(vertex);
            let mut success = 0;
            gl::GetShaderiv(vertex, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut length: i32 = 0;
                gl::GetShaderiv(vertex, gl::INFO_LOG_LENGTH, &mut length);
                let mut v: Vec<u8> = std::vec::Vec::with_capacity(length.try_into().unwrap());
                gl::GetShaderInfoLog(vertex, length, &mut length, v.as_mut_ptr().cast());
                v.set_len(length.try_into().unwrap());
                log::error!("Vertex shader compilation failed: {}", String::from_utf8_lossy(&v));
                gl::DeleteShader(vertex);
                return None;
            }

            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            assert_ne!(fragment, 0);
            gl::ShaderSource(fragment, 1, &(fragment_source.as_ptr().cast()) as *const *const i8, &(fragment_source.len() as i32) as *const i32);
            gl::CompileShader(fragment);
            let mut success = 0;
            gl::GetShaderiv(fragment, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut length: i32 = 0;
                gl::GetShaderiv(fragment, gl::INFO_LOG_LENGTH, &mut length);
                let mut v: Vec<u8> = std::vec::Vec::with_capacity(length.try_into().unwrap());
                gl::GetShaderInfoLog(fragment, length, std::ptr::null_mut(), v.as_mut_ptr().cast());
                v.set_len(length.try_into().unwrap());
                log::error!("Fragment shader compilation failed: {}", String::from_utf8_lossy(&v));
                gl::DeleteShader(vertex);
                gl::DeleteShader(fragment);
                return None;
            }

            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex);
            gl::AttachShader(program, fragment);
            gl::LinkProgram(program);
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);

            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut length: i32 = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut length);
                let mut v: Vec<u8> = std::vec::Vec::with_capacity(length.try_into().unwrap());
                gl::GetProgramInfoLog(program, length, std::ptr::null_mut(), v.as_mut_ptr().cast());
                v.set_len(length.try_into().unwrap());
                log::error!("Shader linking compilation failed: {}", String::from_utf8_lossy(&v));
                return None;
            }

            Some(Self { id: program } )
        }
    }

    fn set_uniform_matrix(&mut self, location: u32, value: nalgebra::Matrix4<f32>) {
        self.bind();
        unsafe {
            gl::UniformMatrix4fv(location.try_into().unwrap(), 1, 1, value.view((0, 0), (4, 4)).as_ptr());
        }
    }

    fn set_uniform_float(&mut self, location: u32, value: f32) {
        self.bind();
        unsafe {
            gl::Uniform1f(location.try_into().unwrap(), value);
        }
    }
    
    fn set_uniform_float2(&mut self, location: u32, value: nalgebra::Vector2<f32>) {
        self.bind();
        unsafe {
            gl::Uniform2f(location.try_into().unwrap(), value.x, value.y);
        }
    }

    fn set_uniform_float3(&mut self, location: u32, value: nalgebra::Vector3<f32>) {
        self.bind();
        unsafe {
            gl::Uniform3f(location.try_into().unwrap(), value.x, value.y, value.z);
        }
    }

    fn set_uniform_float4(&mut self, location: u32, value: nalgebra::Vector4<f32>) {
        self.bind();
        unsafe {
            gl::Uniform4f(location.try_into().unwrap(), value.x, value.y, value.z, value.w);
        }
    }

    fn get_uniform_location(&mut self, name: &str) -> u32 {
        unsafe {
            let mut n = 0;
            gl::GetProgramiv(self.id, gl::ACTIVE_UNIFORM_MAX_LENGTH, &mut n);
            let cname = std::ffi::CString::new(name).unwrap();
            gl::GetUniformLocation(self.id, cname.as_ptr()).try_into().unwrap()
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::UseProgram(0);
            gl::DeleteProgram(self.id);
        }
    }
}