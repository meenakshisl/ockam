use ockam_core::{
    errcode::{Kind, Origin},
    Error,
};

/// Represents the failures that can occur in
/// an Ockam Capability operation
#[derive(Clone, Copy, Debug)]
pub enum CapabilityError {
    /// ToDoOne
    ToDoOne = 1,
    /// ToDoOne
    ToDoTwo,
}

impl ockam_core::compat::error::Error for CapabilityError {}
impl core::fmt::Display for CapabilityError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ToDoOne => write!(f, "TODO One"),
            Self::ToDoTwo => write!(f, "TODO Two"),
        }
    }
}

impl From<CapabilityError> for Error {
    #[track_caller]
    fn from(err: CapabilityError) -> Self {
        use CapabilityError::*;
        let kind = match err {
            ToDoOne => Kind::NotFound,
            _ => Kind::Invalid,
        };

        Error::new(Origin::Capability, kind, err)
    }
}
