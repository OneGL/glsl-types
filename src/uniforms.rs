pub struct Uniform {
    pub name: String,
    pub uniform_type: UniformType,
}

pub enum UniformType {
    Sampler2d,

    Float,
    Vec2,
    Vec3,
    Vec4,

    Int,
    Ivec2,
    Ivec3,
    Ivec4,

    Uint,
    Uvec2,
    Uvec3,
    Uvec4,

    Bool,
    Bvec2,
    Bvec3,
    Bvec4,

    Mat2,
    Mat3,
    Mat4,
}

impl UniformType {
    pub fn from_str(str: &str) -> Result<UniformType, ()> {
        let result = match str {
            "sampler2D" => UniformType::Sampler2d,

            "float" => UniformType::Float,
            "vec2" => UniformType::Vec2,
            "vec3" => UniformType::Vec3,
            "vec4" => UniformType::Vec4,

            "int" => UniformType::Int,
            "ivec2" => UniformType::Ivec2,
            "ivec3" => UniformType::Ivec3,
            "ivec4" => UniformType::Ivec4,

            "uint" => UniformType::Uint,
            "uvec2" => UniformType::Uvec2,
            "uvec3" => UniformType::Uvec3,
            "uvec4" => UniformType::Uvec4,

            "bool" => UniformType::Bool,
            "bvec2" => UniformType::Bvec2,
            "bvec3" => UniformType::Bvec3,
            "bvec4" => UniformType::Bvec4,

            "mat2" => UniformType::Mat2,
            "mat3" => UniformType::Mat3,
            "mat4" => UniformType::Mat4,

            _ => return Err(()),
        };

        return Ok(result);
    }

    pub fn to_ts_type(&self) -> String {
        let result =  match self {
            UniformType::Sampler2d => "WebGLTexture",
    
            UniformType::Float => "number",
            UniformType::Vec2 => "[number, number]",
            UniformType::Vec3 => "[number, number, number]",
            UniformType::Vec4 => "[number, number, number, number]",
    
            UniformType::Int => "number",
            UniformType::Ivec2 => "[number, number]",
            UniformType::Ivec3 => "[number, number, number]",
            UniformType::Ivec4 => "[number, number, number, number]",
    
            UniformType::Uint => "number",
            UniformType::Uvec2 => "[number, number]",
            UniformType::Uvec3 => "[number, number, number]",
            UniformType::Uvec4 => "[number, number, number, number]",
            
            UniformType::Bool => "boolean",
            UniformType::Bvec2 => "[boolean, boolean]",
            UniformType::Bvec3 => "[boolean, boolean, boolean]",
            UniformType::Bvec4 => "[boolean, boolean, boolean, boolean]",
    
            UniformType::Mat2 => "[number, number, number, number]",
            UniformType::Mat3 => "[number, number, number, number, number, number, number, number, number]",
            UniformType::Mat4 => "[number, number, number, number, number, number, number, number, number, number, number, number, number, number, number, number]",
        };
    
        return result.to_string()        
    }
}
