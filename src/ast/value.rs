#[derive(Debug, Clone)]
pub enum Value {
    String(String),
}

impl Value {
    pub fn from_str<A: ToString>(val: A) -> Self {
        Self::String(val.to_string())
    }
}
