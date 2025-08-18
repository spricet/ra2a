use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub mod i32_role_serde {
    use super::*;
    use crate::core::role::Role;
    pub fn serialize<S>(role: &i32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let role = Role::try_from(*role).map_err(serde::ser::Error::custom)?;
        role.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let role = Role::deserialize(deserializer)?;
        let val: i32 = role.into(); // or i32::try_from(role).map_err(D::Error::custom)?
        Ok(val)
    }
}
