use crate::core::artifact::Artifact;
use crate::core::message::Message;
use crate::core::util::{Object, iso8601_timestamp_opt};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Task is the core unit of action for A2A. It has a current status
/// and when results are created for the task they are stored in the
/// artifact. If there are multiple turns for a task, these are stored in
/// history.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct Task {
    #[cfg_attr(feature = "grpc", prost(string, tag = "1"))]
    pub id: String,

    #[cfg_attr(feature = "grpc", prost(string, tag = "2"))]
    pub context_id: String,

    #[cfg_attr(feature = "grpc", prost(message, tag = "3"))]
    pub task_status: Option<TaskStatus>,

    #[cfg_attr(feature = "grpc", prost(repeated, message, tag = "4"))]
    pub artifacts: Vec<Artifact>,

    #[cfg_attr(feature = "grpc", prost(repeated, message, tag = "5"))]
    pub history: Vec<Message>,

    #[cfg_attr(feature = "grpc", prost(message, tag = "6"))]
    pub metadata: Option<Object>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "grpc", derive(prost::Enumeration))]
pub enum TaskState {
    /// Unspecified state
    Unspecified = 0,

    /// Represents the status that acknowledges a task is created
    Submitted = 1,

    /// Represents the status that a task is actively being processed
    Working = 2,

    /// Represents the status a task is finished. This is a terminal state
    Completed = 3,

    /// Represents the status a task is done but failed. This is a terminal state
    Failed = 4,

    /// Represents the status a task was cancelled before it finished.
    /// This is a terminal state.
    Cancelled = 5,

    /// Represents the status that the task requires information to complete.
    /// This is an interrupted state.
    InputRequired = 6,

    /// Represents the status that the agent has decided to not perform the task.
    /// This may be done during initial task creation or later once an agent
    /// has determined it can't or won't proceed. This is a terminal state.
    Rejected = 7,

    /// Represents the state that some authentication is needed from the upstream
    /// client. Authentication is expected to come out-of-band thus this is not
    /// an interrupted or terminal state.
    AuthRequired = 8,
}

/// A container for the status of a task
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct TaskStatus {
    #[cfg_attr(feature = "grpc", prost(enumeration = "TaskState", tag = "1"))]
    pub state: i32,

    #[cfg_attr(feature = "grpc", prost(message, tag = "2"))]
    pub message: Option<Message>,

    #[serde(with = "iso8601_timestamp_opt")]
    #[cfg_attr(feature = "grpc", prost(message, tag = "3"))]
    pub timestamp: Option<Timestamp>,
}

impl Task {
    pub fn new() -> Self {
        Self::new_with_id(Uuid::new_v4().to_string())
    }

    pub fn new_with_id(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            context_id: Uuid::new_v4().to_string(),
            task_status: None,
            artifacts: vec![],
            history: vec![],
            metadata: None,
        }
    }
}
