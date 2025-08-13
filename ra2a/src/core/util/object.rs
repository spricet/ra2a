use prost_types::Struct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// This is a wrapper around `prost_types::Struct` to represent an object in the A2A model.
/// It is both serde capable and can be used in gRPC messages.
#[derive(Clone, PartialEq, Default)]
pub struct Object(pub Struct);

impl Object {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.0.fields.is_empty()
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Serialize for Object {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let json_value = struct_to_json(&self.0);
        json_value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Object {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let json_value = serde_json::Value::deserialize(deserializer)?;
        Ok(Object(json_to_struct(json_value)))
    }
}

#[cfg(feature = "grpc")]
impl prost::Message for Object {
    fn encode_raw(&self, buf: &mut impl bytes::BufMut) {
        self.0.encode_raw(buf)
    }

    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: prost::encoding::WireType,
        buf: &mut impl bytes::Buf,
        ctx: prost::encoding::DecodeContext,
    ) -> Result<(), prost::DecodeError> {
        self.0.merge_field(tag, wire_type, buf, ctx)
    }

    fn encoded_len(&self) -> usize {
        self.0.encoded_len()
    }

    fn clear(&mut self) {
        self.0.clear()
    }
}

fn struct_to_json(s: &Struct) -> serde_json::Value {
    serde_json::Value::Object(
        s.fields
            .iter()
            .map(|(k, v)| (k.clone(), value_to_json(v)))
            .collect(),
    )
}

fn value_to_json(v: &prost_types::Value) -> serde_json::Value {
    use prost_types::value::Kind;
    match &v.kind {
        Some(Kind::NullValue(_)) => serde_json::Value::Null,
        Some(Kind::BoolValue(b)) => serde_json::Value::Bool(*b),
        Some(Kind::NumberValue(n)) => serde_json::json!(n),
        Some(Kind::StringValue(s)) => serde_json::Value::String(s.clone()),
        Some(Kind::ListValue(l)) => {
            serde_json::Value::Array(l.values.iter().map(value_to_json).collect())
        }
        Some(Kind::StructValue(s)) => struct_to_json(s),
        None => serde_json::Value::Null,
    }
}

fn json_to_struct(value: serde_json::Value) -> Struct {
    use prost_types::Struct;

    match value {
        serde_json::Value::Object(map) => Struct {
            fields: map
                .into_iter()
                .map(|(k, v)| (k, json_to_value(v)))
                .collect(),
        },
        _ => Struct {
            fields: Default::default(),
        },
    }
}

fn json_to_value(value: serde_json::Value) -> prost_types::Value {
    use prost_types::{ListValue, Value, value::Kind};

    let kind = match value {
        serde_json::Value::Null => Kind::NullValue(0),
        serde_json::Value::Bool(b) => Kind::BoolValue(b),
        serde_json::Value::Number(n) => Kind::NumberValue(n.as_f64().unwrap_or_default()),
        serde_json::Value::String(s) => Kind::StringValue(s),
        serde_json::Value::Array(arr) => Kind::ListValue(ListValue {
            values: arr.into_iter().map(json_to_value).collect(),
        }),
        serde_json::Value::Object(map) => {
            Kind::StructValue(json_to_struct(serde_json::Value::Object(map)))
        }
    };

    Value { kind: Some(kind) }
}
