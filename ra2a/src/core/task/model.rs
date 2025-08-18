use crate::core::artifact::Artifact;
use crate::core::message::Message;
use crate::core::util::{Object, i32_task_state_serde, iso8601_timestamp_opt};
use bytes::{Buf, BufMut};
use jsonrpsee::core::to_json_raw_value;
use jsonrpsee::core::traits::ToRpcParams;
use prost::DecodeError;
use prost::encoding::{DecodeContext, WireType};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use serde_json::value::RawValue;
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
    pub status: Option<TaskStatus>,

    #[cfg_attr(feature = "grpc", prost(repeated, message, tag = "4"))]
    pub artifacts: Vec<Artifact>,

    #[cfg_attr(feature = "grpc", prost(repeated, message, tag = "5"))]
    pub history: Vec<Message>,

    #[cfg_attr(feature = "grpc", prost(message, tag = "6"))]
    pub metadata: Option<Object>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
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
    #[serde(rename = "input-required")]
    InputRequired = 6,

    /// Represents the status that the agent has decided to not perform the task.
    /// This may be done during initial task creation or later once an agent
    /// has determined it can't or won't proceed. This is a terminal state.
    Rejected = 7,

    /// Represents the state that some authentication is needed from the upstream
    /// client. Authentication is expected to come out-of-band thus this is not
    /// an interrupted or terminal state.
    #[serde(rename = "auth-required")]
    AuthRequired = 8,
}

/// A container for the status of a task
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct TaskStatus {
    #[serde(with = "i32_task_state_serde")]
    #[cfg_attr(feature = "grpc", prost(enumeration = "TaskState", tag = "1"))]
    pub state: i32,

    #[cfg_attr(feature = "grpc", prost(message, tag = "2"))]
    pub message: Option<Message>,

    #[serde(
        default,
        with = "iso8601_timestamp_opt",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "grpc", prost(message, tag = "3"))]
    pub timestamp: Option<Timestamp>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTaskRequest {
    pub id: String,
    pub history_length: Option<i32>,
    pub metadata: Option<Object>,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
pub struct GetTaskGrpcRequest {
    #[cfg_attr(feature = "grpc", prost(string, tag = "1"))]
    pub name: String, // follows the form 'task/{id}'
    #[cfg_attr(feature = "grpc", prost(optional, int32, tag = "2"))]
    pub history_length: Option<i32>,
}

impl From<GetTaskGrpcRequest> for GetTaskRequest {
    fn from(value: GetTaskGrpcRequest) -> Self {
        Self {
            id: value
                .name
                .clone()
                .strip_prefix("task/")
                .unwrap_or(&value.name)
                .to_string(),
            history_length: value.history_length,
            metadata: None,
        }
    }
}

impl From<GetTaskRequest> for GetTaskGrpcRequest {
    fn from(value: GetTaskRequest) -> Self {
        Self {
            name: format!("task/{}", value.id),
            history_length: value.history_length,
        }
    }
}

impl ToRpcParams for GetTaskRequest {
    fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, Error> {
        to_json_raw_value(&self).map(Some)
    }
}

impl Task {
    pub fn new() -> Self {
        Self::new_with_id(Uuid::new_v4().to_string())
    }

    pub fn new_with_id(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            context_id: Uuid::new_v4().to_string(),
            status: None,
            artifacts: vec![],
            history: vec![],
            metadata: None,
        }
    }
}

impl TaskStatus {
    pub fn default_submitted() -> Self {
        Self {
            state: TaskState::Submitted.into(),
            message: None,
            timestamp: None,
        }
    }
}

impl TaskState {
    pub fn into_i32(self) -> i32 {
        self.into()
    }
}
