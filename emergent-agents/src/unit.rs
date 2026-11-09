use serde::{Deserialize, Serialize};

use crate::{
    unit_definition::{CONVERT_INTO_BYTES, F32, SPLIT_WITH_SPACE, UnitDefinition},
    unit_id::UnitId,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum Unit {
    SplitWithSpace { id: UnitId },
    ConvertIntoBytes { id: UnitId },
    F32 { id: UnitId, value: f32 },
}

impl Unit {
    pub(crate) fn split_with_space() -> Self {
        Self::SplitWithSpace { id: UnitId::new() }
    }

    pub(crate) fn convert_into_bytes() -> Self {
        Self::ConvertIntoBytes { id: UnitId::new() }
    }

    pub(crate) fn f32(value: f32) -> Self {
        Self::F32 {
            id: UnitId::new(),
            value,
        }
    }

    pub(crate) const fn id(&self) -> UnitId {
        match self {
            Self::SplitWithSpace { id } | Self::ConvertIntoBytes { id } | Self::F32 { id, .. } => {
                *id
            }
        }
    }

    pub(crate) const fn definition(&self) -> &'static UnitDefinition {
        match self {
            Self::SplitWithSpace { .. } => &SPLIT_WITH_SPACE,
            Self::ConvertIntoBytes { .. } => &CONVERT_INTO_BYTES,
            Self::F32 { .. } => &F32,
        }
    }
}
