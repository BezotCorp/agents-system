use crate::unit_id::UnitId;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BrainCognitiveFailure {
    MissingStart,

    MissingUuidTarget {
        start: UnitId,
        target: UnitId,
    },

    InvalidUsizeTarget {
        start: UnitId,
        target: usize,
    },

    InvalidOrderBy {
        start: UnitId,
        reason: String,
    },

    IndexOutOfBounds {
        index: usize,
    },

    NoReachableOutput,

    InvalidInput {
        unit: UnitId,
        expected: &'static str,
        received: &'static str,
    },

    ExecutionBudgetExceeded {
        limit: usize,
    },
}
