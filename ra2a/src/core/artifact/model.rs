use crate::core::part::Part;
use crate::core::util::Object;
use serde::{Deserialize, Serialize};

/// Artifacts are the container for task completed results. These are similar
/// to Messages but are intended to be the product of a task, as opposed to
/// point-to-point communication.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct Artifact {
    #[cfg_attr(feature = "grpc", prost(string, tag = "1"))]
    pub artifact_id: String,

    #[cfg_attr(feature = "grpc", prost(string, tag = "3"))]
    pub name: String,

    #[cfg_attr(feature = "grpc", prost(string, tag = "4"))]
    pub description: String,

    #[cfg_attr(feature = "grpc", prost(repeated, message, tag = "5"))]
    pub parts: Vec<Part>,

    #[cfg_attr(feature = "grpc", prost(message, tag = "6"))]
    pub metadata: Option<Object>,

    #[cfg_attr(feature = "grpc", prost(repeated, string, tag = "7"))]
    pub extensions: Vec<String>,
}
