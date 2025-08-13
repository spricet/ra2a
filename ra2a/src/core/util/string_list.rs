use derive_builder::Builder;
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Clone, PartialEq, Eq, Builder)]
#[builder(setter(into, strip_option))]
#[cfg_attr(feature = "grpc", derive(prost::Message))]
#[cfg_attr(not(feature = "grpc"), derive(Debug))]
pub struct StringList {
    #[cfg_attr(feature = "grpc", prost(repeated, string, tag = "1"))]
    pub list: Vec<String>,
}

impl Serialize for StringList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.list.len()))?;
        for item in &self.list {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for StringList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec = Vec::<String>::deserialize(deserializer)?;
        Ok(StringList { list: vec })
    }
}
