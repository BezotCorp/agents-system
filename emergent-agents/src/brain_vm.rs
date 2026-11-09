use std::{collections::HashSet, fs, path::Path};

use crate::{
    brain_genome::BrainGenome, brain_runtime::BrainRuntime, brain_vm_error::BrainVmError,
    unit_id::UnitId,
};

pub(crate) struct BrainVm {
    runtime: BrainRuntime,
}

impl BrainVm {
    pub(crate) fn from_ron(path: impl AsRef<Path>) -> Result<Self, BrainVmError> {
        let content =
            fs::read_to_string(path).map_err(|error| BrainVmError::Io(error.to_string()))?;

        let genome: BrainGenome =
            ron::from_str(&content).map_err(|error| BrainVmError::Ron(error.to_string()))?;

        Self::from_genome(genome)
    }

    pub(crate) fn from_bin(path: impl AsRef<Path>) -> Result<Self, BrainVmError> {
        let bytes = fs::read(path).map_err(|error| BrainVmError::Io(error.to_string()))?;

        let config = bincode_next::config::standard();

        let (genome, _): (BrainGenome, usize) =
            bincode_next::serde::decode_from_slice(&bytes, config)
                .map_err(|error| BrainVmError::Bin(error.to_string()))?;

        Self::from_genome(genome)
    }

    pub(crate) fn from_genome(genome: BrainGenome) -> Result<Self, BrainVmError> {
        Self::validate(&genome)?;

        Ok(Self {
            runtime: BrainRuntime::from_genome(genome),
        })
    }

    pub(crate) fn runtime(&self) -> &BrainRuntime {
        &self.runtime
    }

    pub(crate) fn runtime_mut(&mut self) -> &mut BrainRuntime {
        &mut self.runtime
    }

    pub(crate) fn save_ron(&self, path: impl AsRef<Path>) -> Result<(), BrainVmError> {
        let genome = self.runtime.snapshot();

        let content = ron::ser::to_string_pretty(&genome, ron::ser::PrettyConfig::default())
            .map_err(|error| BrainVmError::Ron(error.to_string()))?;

        fs::write(path, content).map_err(|error| BrainVmError::Io(error.to_string()))
    }

    pub(crate) fn save_bin(&self, path: impl AsRef<Path>) -> Result<(), BrainVmError> {
        let genome = self.runtime.snapshot();
        let config = bincode_next::config::standard();

        let bytes = bincode_next::serde::encode_to_vec(&genome, config)
            .map_err(|error| BrainVmError::Bin(error.to_string()))?;

        fs::write(path, bytes).map_err(|error| BrainVmError::Io(error.to_string()))
    }

    fn validate(genome: &BrainGenome) -> Result<(), BrainVmError> {
        let mut ids = HashSet::<UnitId>::new();

        for unit in genome.units() {
            if !ids.insert(unit.id()) {
                return Err(BrainVmError::DuplicateUnitId(unit.id()));
            }
        }

        Ok(())
    }
}
