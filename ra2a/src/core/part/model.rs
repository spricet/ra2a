use crate::core::util::Object;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct Part {
    #[serde(flatten)]
    #[cfg_attr(feature = "grpc", prost(oneof = "PartBase", tags = "1, 2, 3"))]
    pub part: Option<PartBase>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
#[cfg_attr(feature = "grpc", derive(prost::Oneof))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub enum PartBase {
    #[serde(with = "text_part_base_interop_serde")]
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

mod text_part_base_interop_serde {
    use serde::ser::SerializeStruct;
    use serde::{Deserialize, Deserializer, Serializer};

    #[derive(Deserialize)]
    struct Wrapper {
        text: String,
    }

    pub fn serialize<S>(role: &String, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("TextPart", 1)?;
        s.serialize_field("text", role)?;
        s.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let Wrapper { text } = Wrapper::deserialize(deserializer)?;
        Ok(text)
    }
}
