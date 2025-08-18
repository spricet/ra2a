use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "grpc", derive(prost::Enumeration))]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// Unspecified role
    Unspecified = 0,
    /// Refers to communication from the client to the server.
    User = 1,
    /// Refers to communication from the server to the client.
    Agent = 2,
}

impl Role {
    pub fn unspecified_i32() -> i32 {
        Self::Unspecified.into()
    }

    pub fn is_unspecified(val: &i32) -> bool {
        match Role::try_from(*val) {
            Ok(role) => role == Self::Unspecified,
            Err(_) => false,
        }
    }
}
