#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BrainValue {
    Text(String),

    TextPair { first: String, rest: String },

    Bytes(Vec<u8>),

    F32(f32),
}

impl BrainValue {
    pub(crate) const fn type_name(&self) -> &'static str {
        match self {
            Self::Text(_) => "text",
            Self::TextPair { .. } => "text_pair",
            Self::Bytes(_) => "bytes",
            Self::F32(_) => "f32",
        }
    }
}
