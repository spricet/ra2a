use crate::core::part::Part;
use crate::core::push_notification::PushNotificationConfig;
#[cfg(feature = "grpc")]
use crate::core::role::Role;
use crate::core::task::Task;
use crate::core::util::Object;
use jsonrpsee::core::to_json_raw_value;
use jsonrpsee::core::traits::ToRpcParams;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use serde_json::value::RawValue;

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

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct SendMessageRequest {
    #[cfg_attr(feature = "grpc", prost(message, tag = "1"))]
    pub message: Option<Message>,

    #[cfg_attr(feature = "grpc", prost(message, tag = "2"))]
    pub configuration: Option<SendMessageConfiguration>,

    #[cfg_attr(feature = "grpc", prost(message, tag = "3"))]
    pub metadata: Option<Object>,
}

/// Configuration of a send message request.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct SendMessageConfiguration {
    /// The output modes that the agent is expected to respond with.
    #[cfg_attr(feature = "grpc", prost(repeated, string, tag = "1"))]
    pub accepted_output_modes: Vec<String>,

    /// A configuration of a webhook that can be used to receive updates
    #[cfg_attr(feature = "grpc", prost(message, tag = "2"))]
    pub push_notification: Option<PushNotificationConfig>,

    /// The maximum number of messages to include in the history. if 0, the
    /// history will be unlimited.
    #[cfg_attr(feature = "grpc", prost(int32, tag = "3"))]
    pub history_length: i32,

    /// If true, the message will be blocking until the task is completed. If
    /// false, the message will be non-blocking and the task will be returned
    /// immediately. It is the caller's responsibility to check for any task
    /// updates.
    #[cfg_attr(feature = "grpc", prost(bool, tag = "4"))]
    pub blocking: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct SendMessageResponse {
    #[cfg_attr(
        feature = "grpc",
        prost(oneof = "SendMessageResponsePayload", tags = "1, 2")
    )]
    pub payload: Option<SendMessageResponsePayload>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Oneof))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub enum SendMessageResponsePayload {
    #[cfg_attr(feature = "grpc", prost(message, tag = "1"))]
    Task(Task),

    #[cfg_attr(feature = "grpc", prost(message, tag = "2"))]
    Message(Message),
}

impl ToRpcParams for SendMessageRequest {
    fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, Error> {
        to_json_raw_value(&self).map(Some)
    }
}
