use crate::core::part::Part;
#[cfg(feature = "grpc")]
use crate::core::role::Role;
use crate::core::util::Object;
use serde::{Deserialize, Serialize};

/// Message is one unit of communication between client and server. It is
/// associated with a context and optionally a task. Since the server is
/// responsible for the context definition, it must always provide a context_id
/// in its messages. The client can optionally provide the context_id if it
/// knows the context to associate the message to. Similarly for task_id,
/// except the server decides if a task is created and whether to include the
/// task_id.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct Message {
    #[cfg_attr(feature = "grpc", prost(string, tag = "1"))]
    pub message_id: String,

    #[cfg_attr(feature = "grpc", prost(string, tag = "2"))]
    pub context_id: String,

    #[cfg_attr(feature = "grpc", prost(string, tag = "3"))]
    pub task_id: String,

    #[cfg_attr(feature = "grpc", prost(enumeration = "Role", tag = "4"))]
    pub role: i32,

    #[cfg_attr(feature = "grpc", prost(repeated, message, tag = "5"))]
    pub content: Vec<Part>,

    #[cfg_attr(feature = "grpc", prost(message, tag = "6"))]
    pub metadata: Option<Object>,

    #[cfg_attr(feature = "grpc", prost(repeated, string, tag = "7"))]
    pub extensions: Vec<String>,
}
