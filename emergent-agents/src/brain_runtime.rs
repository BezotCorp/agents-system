use crate::{brain_genome::BrainGenome, unit::Unit, unit_id::UnitId};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BrainRuntime {
    units: Vec<Unit>,
}

impl BrainRuntime {
    pub(crate) fn from_genome(genome: BrainGenome) -> Self {
        Self {
            units: genome.units().to_vec(),
        }
    }

    pub(crate) fn units(&self) -> &[Unit] {
        &self.units
    }

    pub(crate) fn unit(&self, id: UnitId) -> Option<&Unit> {
        self.units.iter().find(|unit| unit.id() == id)
    }

    pub(crate) fn unit_mut(&mut self, id: UnitId) -> Option<&mut Unit> {
        self.units.iter_mut().find(|unit| unit.id() == id)
    }

    pub(crate) fn snapshot(&self) -> BrainGenome {
        BrainGenome::new(self.units.clone())
    }
}
