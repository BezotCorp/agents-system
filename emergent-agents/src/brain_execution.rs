use crate::{brain_cognitive_failure::BrainCognitiveFailure, brain_value::BrainValue};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BrainExecution {
    Output {
        value: BrainValue,
        steps: usize,
    },

    CognitiveFailure {
        failure: BrainCognitiveFailure,
        steps: usize,
    },
}
