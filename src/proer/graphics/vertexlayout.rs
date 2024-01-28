#[derive(Copy, Clone)]
pub enum VertexAttributeType {
    Float,
    Float2,
    Float3,
    Float4,
    Int,
}

impl VertexAttributeType {
    pub fn size(self) -> u32 {
        match self {
            Self::Float => 4,
            Self::Float2 => 8,
            Self::Float3 => 12,
            Self::Float4 => 16,
            Self::Int => 4,
        }
    }

    pub fn amount(self) -> u32 {
        match self {
            Self::Float => 1,
            Self::Float2 => 2,
            Self::Float3 => 3,
            Self::Float4 => 4,
            Self::Int => 1,
        }       
    }
}

#[derive(Clone)]
pub struct VertexAtribute {
    pub name: &'static str,
    pub datatype: VertexAttributeType,
}


