use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "grpc", derive(prost::Enumeration))]
pub enum Role {
    /// Unspecified role
    Unspecified = 0,
    /// Refers to communication from the client to the server.
    User = 1,
    /// Refers to communication from the server to the client.
    Agent = 2,
}
