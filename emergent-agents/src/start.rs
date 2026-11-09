use serde::{Deserialize, Serialize};

use crate::unit_id::UnitId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Start {
    Marker,
    Usize(usize),
    Uuid(UnitId),
}
