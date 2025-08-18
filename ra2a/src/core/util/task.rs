use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub mod i32_task_state_serde {
    use super::*;
    use crate::core::task::TaskState;
    pub fn serialize<S>(state: &i32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let role = TaskState::try_from(*state).map_err(serde::ser::Error::custom)?;
        role.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let state = TaskState::deserialize(deserializer)?;
        let val: i32 = state.into();
        Ok(val)
    }
}
