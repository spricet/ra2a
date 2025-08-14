use jsonrpsee::core::ClientError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum A2AErrorCode {
    TaskNotFound = -32001,
    TaskNotCancelable = -32002,
    PushNotificationNotSupported = -32003,
    UnsupportedOperation = -32004,
    ContentTypeNotSupported = -32005,
    InvalidAgentResponse = -32006,
    AuthenticatedExtendedCardNotConfigured = -32007,
}

#[derive(Debug, Error)]
pub enum A2AError {
    #[error(transparent)]
    Protocol(#[from] A2AProtocolError),

    #[error(transparent)]
    Transport(#[from] A2ATransportError),
}

#[derive(Debug, Error)]
pub enum A2AProtocolError {
    /// The specified task id does not correspond to an existing or active task.
    /// It might be invalid, expired, or already completed and purged.
    #[error("Task not found")]
    TaskNotFound { id: String, code: A2AErrorCode },

    /// An attempt was made to cancel a task that is not in a cancelable state
    /// (e.g., it has already reached a terminal state like completed, failed, or canceled).
    #[error("Task cannot be canceled")]
    TaskNotCancelable { id: String, code: A2AErrorCode },

    /// Client attempted to use push notification features (e.g., tasks/pushNotificationConfig/set)
    /// but the server agent does not support them (i.e., AgentCard.capabilities.pushNotifications is false).
    #[error("Push Notification is not supported")]
    PushNotificationNotSupported { code: A2AErrorCode },

    /// The requested operation or a specific aspect of it (perhaps implied by parameters)
    /// is not supported by this server agent implementation. Broader than just method not found.
    #[error("This operation is not supported")]
    UnsupportedOperation { code: A2AErrorCode },

    /// A Media Type provided in the request's message.parts (or implied for an artifact)
    /// is not supported by the agent or the specific skill being invoked.
    #[error("Incompatible content types")]
    ContentTypeNotSupported { code: A2AErrorCode },

    /// Agent generated an invalid response for the requested method
    #[error("Invalid agent response type")]
    InvalidAgentResponse { code: A2AErrorCode },

    /// The agent does not have an Authenticated Extended Card configured.
    #[error("Authenticated Extended Card not configured")]
    AuthenticatedExtendedCardNotConfigured { code: A2AErrorCode },
}

#[derive(Debug, Error)]
pub enum A2ATransportError {
    #[error("Missing payload")]
    MissingPayload,

    #[cfg(feature = "grpc")]
    #[error("GRPC")]
    Grcp(#[from] tonic::Status),

    #[error("Json RPC")]
    JsonRpc(#[from] ClientError),
}

impl A2AProtocolError {
    pub fn task_not_found(id: String) -> Self {
        A2AProtocolError::TaskNotFound {
            id,
            code: A2AErrorCode::TaskNotFound,
        }
    }

    pub fn task_not_cancelable(id: String) -> Self {
        A2AProtocolError::TaskNotCancelable {
            id,
            code: A2AErrorCode::TaskNotCancelable,
        }
    }

    pub fn push_notification_not_supported() -> Self {
        A2AProtocolError::PushNotificationNotSupported {
            code: A2AErrorCode::PushNotificationNotSupported,
        }
    }

    pub fn unsupported_operation() -> Self {
        A2AProtocolError::UnsupportedOperation {
            code: A2AErrorCode::UnsupportedOperation,
        }
    }

    pub fn content_type_not_supported() -> Self {
        A2AProtocolError::ContentTypeNotSupported {
            code: A2AErrorCode::ContentTypeNotSupported,
        }
    }

    pub fn invalid_agent_response() -> Self {
        A2AProtocolError::InvalidAgentResponse {
            code: A2AErrorCode::InvalidAgentResponse,
        }
    }

    pub fn authenticated_extended_card_not_configured() -> Self {
        A2AProtocolError::AuthenticatedExtendedCardNotConfigured {
            code: A2AErrorCode::AuthenticatedExtendedCardNotConfigured,
        }
    }
}

#[cfg(feature = "grpc")]
impl From<tonic::Status> for A2AError {
    fn from(value: tonic::Status) -> Self {
        Self::Transport(A2ATransportError::Grcp(value))
    }
}

impl From<ClientError> for A2AError {
    fn from(value: ClientError) -> Self {
        if let ClientError::Custom(_value) = &value {
            todo!("this probably encapsulates the protocol error, so we should parse it");
        }
        Self::Transport(A2ATransportError::JsonRpc(value))
    }
}
