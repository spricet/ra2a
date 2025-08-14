use crate::core::util::Object;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct Part {
    #[serde(flatten)]
    #[cfg_attr(feature = "grpc", prost(oneof = "PartType", tags = "1, 2, 3"))]
    pub part: Option<PartType>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[cfg_attr(feature = "grpc", derive(prost::Oneof))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub enum PartType {
    #[cfg_attr(feature = "grpc", prost(string, tag = "1"))]
    Text(String),

    #[cfg_attr(feature = "grpc", prost(message, tag = "2"))]
    File(FilePart),

    #[cfg_attr(feature = "grpc", prost(message, tag = "3"))]
    Data(DataPart),
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct FilePart {
    #[serde(flatten)]
    #[cfg_attr(feature = "grpc", prost(oneof = "File", tags = "1, 2"))]
    pub file: Option<File>,

    #[cfg_attr(feature = "grpc", prost(string, tag = "3"))]
    pub mime_type: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[cfg_attr(feature = "grpc", derive(prost::Oneof))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub enum File {
    #[cfg_attr(feature = "grpc", prost(string, tag = "1"))]
    FileWithUri(String),
    #[cfg_attr(feature = "grpc", prost(bytes = "vec", tag = "2"))]
    FileWithBytes(Vec<u8>),
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct DataPart {
    #[cfg_attr(feature = "grpc", prost(message, tag = "1"))]
    pub data: Option<Object>,
}
