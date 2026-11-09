use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::unit_id::UnitId;

#[derive(Debug)]
pub(crate) enum BrainVmError {
    Io(String),
    Ron(String),
    Bin(String),
    DuplicateUnitId(UnitId),
}

impl Display for BrainVmError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => {
                write!(formatter, "brain I/O error: {error}")
            }
            Self::Ron(error) => {
                write!(formatter, "brain RON error: {error}")
            }
            Self::Bin(error) => {
                write!(formatter, "brain binary error: {error}")
            }
            Self::DuplicateUnitId(id) => {
                write!(formatter, "duplicate brain unit UUID: {}", id.value(),)
            }
        }
    }
}

impl Error for BrainVmError {}
