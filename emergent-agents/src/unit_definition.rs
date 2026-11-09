#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct UnitDefinition {
    name: &'static str,
}

impl UnitDefinition {
    pub(crate) const fn new(name: &'static str) -> Self {
        Self { name }
    }

    pub(crate) const fn name(self) -> &'static str {
        self.name
    }
}

pub(crate) const SPLIT_WITH_SPACE: UnitDefinition = UnitDefinition::new("split_with_space");

pub(crate) const CONVERT_INTO_BYTES: UnitDefinition = UnitDefinition::new("convert_into_bytes");

pub(crate) const F32: UnitDefinition = UnitDefinition::new("f32");
