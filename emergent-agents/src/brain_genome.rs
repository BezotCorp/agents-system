use serde::{Deserialize, Serialize};

use crate::unit::Unit;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct BrainGenome {
    units: Vec<Unit>,
}

impl BrainGenome {
    pub(crate) fn new(units: Vec<Unit>) -> Self {
        Self { units }
    }

    pub(crate) fn units(&self) -> &[Unit] {
        &self.units
    }
}
