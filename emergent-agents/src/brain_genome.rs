use serde::{Deserialize, Serialize};

use crate::{unit::Unit, unit_id::UnitId};

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

    pub(crate) fn unit(&self, id: UnitId) -> Option<&Unit> {
        self.units.iter().find(|unit| unit.id() == id)
    }

    pub(crate) fn unit_at(&self, index: usize) -> Option<&Unit> {
        self.units.get(index)
    }

    pub(crate) fn index_of(&self, id: UnitId) -> Option<usize> {
        self.units.iter().position(|unit| unit.id() == id)
    }

    pub(crate) fn set_f32(&mut self, id: UnitId, value: f32) -> bool {
        let Some(unit) = self.units.iter_mut().find(|unit| unit.id() == id) else {
            return false;
        };

        unit.set_f32(value)
    }

    pub(crate) fn set_order_by_value(
        &mut self,
        id: UnitId,
        name: impl Into<String>,
        value: f64,
    ) -> bool {
        let Some(unit) = self.units.iter_mut().find(|unit| unit.id() == id) else {
            return false;
        };

        unit.set_order_by_value(name, value)
    }
}
