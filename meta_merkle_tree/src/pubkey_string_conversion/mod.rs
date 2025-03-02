
use std::str::FromStr;

use serde::{self, Deserialize, Deserializer, Serializer};
use solana_program::pubkey::Pubkey;

pub fn serialize<S>(pubkey: &Pubkey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&pubkey.to_string())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Pubkey::from_str(&s).map_err(serde::de::Error::custom)
}
