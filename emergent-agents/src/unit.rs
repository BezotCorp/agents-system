use serde::{Deserialize, Serialize};

use crate::{
    brain_cognitive_failure::BrainCognitiveFailure, brain_value::BrainValue, order_by::OrderBy,
    start::Start, unit_id::UnitId,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) enum Unit {
    Start {
        id: UnitId,
        start: Start,
        order_by: OrderBy,
    },

    SplitWithSpace {
        id: UnitId,
    },

    ConvertIntoBytes {
        id: UnitId,
    },

    F32 {
        id: UnitId,
        value: f32,
    },

    Output {
        id: UnitId,
    },

    Stop {
        id: UnitId,
    },
}

impl Unit {
    pub(crate) fn start(start: Start, order_by: OrderBy) -> Self {
        Self::Start {
            id: UnitId::new(),
            start,
            order_by,
        }
    }

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

    pub(crate) fn output() -> Self {
        Self::Output { id: UnitId::new() }
    }

    pub(crate) fn stop() -> Self {
        Self::Stop { id: UnitId::new() }
    }

    pub(crate) const fn id(&self) -> UnitId {
        match self {
            Self::Start { id, .. }
            | Self::SplitWithSpace { id }
            | Self::ConvertIntoBytes { id }
            | Self::F32 { id, .. }
            | Self::Output { id }
            | Self::Stop { id } => *id,
        }
    }

    pub(crate) const fn start_configuration(&self) -> Option<(Start, &OrderBy)> {
        match self {
            Self::Start {
                start, order_by, ..
            } => Some((*start, order_by)),

            _ => None,
        }
    }

    pub(crate) const fn is_output(&self) -> bool {
        matches!(self, Self::Output { .. })
    }

    pub(crate) const fn is_stop(&self) -> bool {
        matches!(self, Self::Stop { .. })
    }

    pub(crate) fn set_f32(&mut self, value: f32) -> bool {
        match self {
            Self::F32 { value: current, .. } => {
                *current = value;
                true
            }

            _ => false,
        }
    }

    pub(crate) fn set_order_by_value(&mut self, name: impl Into<String>, value: f64) -> bool {
        match self {
            Self::Start { order_by, .. } => {
                order_by.set_value(name, value);
                true
            }

            _ => false,
        }
    }

    pub(crate) fn execute(&self, input: BrainValue) -> Result<BrainValue, BrainCognitiveFailure> {
        match self {
            Self::SplitWithSpace { id } => match input {
                BrainValue::Text(text) => {
                    let (first, rest) = match text.split_once(' ') {
                        Some((first, rest)) => (first.to_owned(), rest.to_owned()),

                        None => (text, String::new()),
                    };

                    Ok(BrainValue::TextPair { first, rest })
                }

                received => Err(BrainCognitiveFailure::InvalidInput {
                    unit: *id,
                    expected: "text",
                    received: received.type_name(),
                }),
            },

            Self::ConvertIntoBytes { id } => match input {
                BrainValue::Text(text) => Ok(BrainValue::Bytes(text.into_bytes())),

                received => Err(BrainCognitiveFailure::InvalidInput {
                    unit: *id,
                    expected: "text",
                    received: received.type_name(),
                }),
            },

            Self::F32 { value, .. } => Ok(BrainValue::F32(*value)),

            Self::Start { .. } | Self::Output { .. } | Self::Stop { .. } => Ok(input),
        }
    }
}
