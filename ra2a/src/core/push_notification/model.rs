use serde::{Deserialize, Serialize};

/// Defines the configuration for setting up push notifications for task updates.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct PushNotificationConfig {
    /// A unique id for this push notification.
    #[cfg_attr(feature = "grpc", prost(string, tag = "1"))]
    pub id: String,

    /// Url to send the notification to.
    #[cfg_attr(feature = "grpc", prost(string, tag = "2"))]
    pub url: String,

    /// Token unique for this task/session
    #[cfg_attr(feature = "grpc", prost(string, tag = "3"))]
    pub token: String,

    /// Optional authentication details for the agent to use when calling the notification URL.
    #[cfg_attr(feature = "grpc", prost(message, tag = "4"))]
    pub authentication: Option<PushNotificationAuthenticationInfo>,
}

/// Defines authentication details for a push notification endpoint.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct PushNotificationAuthenticationInfo {
    /// A list of supported authentication schemes (e.g., 'Basic', 'Bearer').
    #[cfg_attr(feature = "grpc", prost(repeated, string, tag = "1"))]
    pub schemes: Vec<String>,

    /// Optional credentials required by the push notification endpoint.
    #[cfg_attr(feature = "grpc", prost(string, tag = "2"))]
    pub credentials: String,
}

/// A container associating a push notification configuration with a specific task.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct TaskPushNotificationConfig {
    /// The ID of the task.
    #[cfg_attr(feature = "grpc", prost(string, tag = "1"))]
    pub task_id: String,

    /// The push notification configuration for this task.
    #[cfg_attr(feature = "grpc", prost(message, tag = "2"))]
    pub push_notification_config: Option<PushNotificationConfig>,
}
