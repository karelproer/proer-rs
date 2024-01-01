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
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}