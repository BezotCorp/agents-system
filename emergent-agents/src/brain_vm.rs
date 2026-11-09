use std::{collections::HashSet, fs, path::Path};

use crate::{
    brain_cognitive_failure::BrainCognitiveFailure, brain_execution::BrainExecution,
    brain_genome::BrainGenome, brain_value::BrainValue, brain_vm_error::BrainVmError,
    order_by::OrderBy, start::Start, unit_id::UnitId,
};

pub(crate) struct BrainVm {
    genome: BrainGenome,
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

        Ok(Self { genome })
    }
    pub(crate) fn genome(&self) -> &BrainGenome {
        &self.genome
    }

    pub(crate) fn genome_mut(&mut self) -> &mut BrainGenome {
        &mut self.genome
    }

    pub(crate) fn execute(&self, input: BrainValue, max_steps: usize) -> BrainExecution {
        let Some(start_index) = self.first_start_index() else {
            return BrainExecution::CognitiveFailure {
                failure: BrainCognitiveFailure::MissingStart,
                steps: 0,
            };
        };

        if let Err(failure) = self.has_reachable_output(start_index, max_steps) {
            return BrainExecution::CognitiveFailure { failure, steps: 0 };
        }

        self.execute_from_start(start_index, input, max_steps)
    }

    pub(crate) fn save_ron(&self, path: impl AsRef<Path>) -> Result<(), BrainVmError> {
        let content = ron::ser::to_string_pretty(&self.genome, ron::ser::PrettyConfig::default())
            .map_err(|error| BrainVmError::Ron(error.to_string()))?;

        fs::write(path, content).map_err(|error| BrainVmError::Io(error.to_string()))
    }

    pub(crate) fn save_bin(&self, path: impl AsRef<Path>) -> Result<(), BrainVmError> {
        let config = bincode_next::config::standard();

        let bytes = bincode_next::serde::encode_to_vec(&self.genome, config)
            .map_err(|error| BrainVmError::Bin(error.to_string()))?;

        fs::write(path, bytes).map_err(|error| BrainVmError::Io(error.to_string()))
    }

    fn first_start_index(&self) -> Option<usize> {
        self.genome
            .units()
            .iter()
            .position(|unit| unit.start_configuration().is_some())
    }

    fn execute_from_start(
        &self,
        start_index: usize,
        input: BrainValue,
        max_steps: usize,
    ) -> BrainExecution {
        let Some(start_unit) = self.genome.unit_at(start_index) else {
            return BrainExecution::CognitiveFailure {
                failure: BrainCognitiveFailure::MissingStart,
                steps: 0,
            };
        };

        let Some((start, order_by)) = start_unit.start_configuration() else {
            return BrainExecution::CognitiveFailure {
                failure: BrainCognitiveFailure::MissingStart,
                steps: 0,
            };
        };

        let mut active_start = start_unit.id();
        let mut active_order_by = order_by;

        let mut current_index =
            match self.next_from_start(start_index, start, active_order_by, active_start) {
                Ok(index) => index,

                Err(failure) => {
                    return BrainExecution::CognitiveFailure { failure, steps: 0 };
                }
            };

        let mut value = input;
        let mut steps = 0;

        loop {
            if steps >= max_steps {
                return BrainExecution::CognitiveFailure {
                    failure: BrainCognitiveFailure::ExecutionBudgetExceeded { limit: max_steps },
                    steps,
                };
            }

            let Some(unit) = self.genome.unit_at(current_index) else {
                return BrainExecution::CognitiveFailure {
                    failure: BrainCognitiveFailure::IndexOutOfBounds {
                        index: current_index,
                    },
                    steps,
                };
            };

            steps += 1;

            if let Some((start, order_by)) = unit.start_configuration() {
                active_start = unit.id();
                active_order_by = order_by;

                current_index =
                    match self.next_from_start(current_index, start, active_order_by, active_start)
                    {
                        Ok(index) => index,

                        Err(failure) => {
                            return BrainExecution::CognitiveFailure { failure, steps };
                        }
                    };

                continue;
            }

            if unit.is_output() {
                return BrainExecution::Output { value, steps };
            }

            if unit.is_stop() {
                return BrainExecution::CognitiveFailure {
                    failure: BrainCognitiveFailure::NoReachableOutput,
                    steps,
                };
            }

            value = match unit.execute(value) {
                Ok(value) => value,

                Err(failure) => {
                    return BrainExecution::CognitiveFailure { failure, steps };
                }
            };

            current_index = match active_order_by.next_index(current_index) {
                Ok(index) => index,

                Err(reason) => {
                    return BrainExecution::CognitiveFailure {
                        failure: BrainCognitiveFailure::InvalidOrderBy {
                            start: active_start,
                            reason,
                        },
                        steps,
                    };
                }
            };
        }
    }

    fn has_reachable_output(
        &self,
        start_index: usize,
        max_steps: usize,
    ) -> Result<(), BrainCognitiveFailure> {
        let Some(start_unit) = self.genome.unit_at(start_index) else {
            return Err(BrainCognitiveFailure::MissingStart);
        };

        let Some((start, order_by)) = start_unit.start_configuration() else {
            return Err(BrainCognitiveFailure::MissingStart);
        };

        let mut active_start = start_unit.id();
        let mut active_order_by = order_by;

        let mut current_index =
            self.next_from_start(start_index, start, active_order_by, active_start)?;

        let mut visited = HashSet::<(usize, UnitId)>::new();

        let mut steps = 0;

        loop {
            if steps >= max_steps {
                return Err(BrainCognitiveFailure::ExecutionBudgetExceeded { limit: max_steps });
            }

            if !visited.insert((current_index, active_start)) {
                return Err(BrainCognitiveFailure::NoReachableOutput);
            }

            let Some(unit) = self.genome.unit_at(current_index) else {
                return Err(BrainCognitiveFailure::IndexOutOfBounds {
                    index: current_index,
                });
            };

            steps += 1;

            if unit.is_output() {
                return Ok(());
            }

            if unit.is_stop() {
                return Err(BrainCognitiveFailure::NoReachableOutput);
            }

            if let Some((start, order_by)) = unit.start_configuration() {
                active_start = unit.id();
                active_order_by = order_by;

                current_index =
                    self.next_from_start(current_index, start, active_order_by, active_start)?;

                continue;
            }

            current_index = active_order_by
                .next_index(current_index)
                .map_err(|reason| BrainCognitiveFailure::InvalidOrderBy {
                    start: active_start,
                    reason,
                })?;
        }
    }

    fn next_from_start(
        &self,
        marker_index: usize,
        start: Start,
        order_by: &OrderBy,
        start_id: UnitId,
    ) -> Result<usize, BrainCognitiveFailure> {
        let reference_index =
            match start {
                Start::Marker => marker_index,

                Start::Usize(index) => {
                    if self.genome.unit_at(index).is_none() {
                        return Err(BrainCognitiveFailure::InvalidUsizeTarget {
                            start: start_id,
                            target: index,
                        });
                    }

                    index
                }

                Start::Uuid(target) => self.genome.index_of(target).ok_or(
                    BrainCognitiveFailure::MissingUuidTarget {
                        start: start_id,
                        target,
                    },
                )?,
            };

        order_by.next_index(reference_index).map_err(|reason| {
            BrainCognitiveFailure::InvalidOrderBy {
                start: start_id,
                reason,
            }
        })
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
