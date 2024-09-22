extern crate serde;
use base64;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    key: [u8; 32],
}

pub fn from_base64<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    String::deserialize(deserializer)
        .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
        .and_then(|vec| {
            <[u8; 32]>::try_from(vec)
                .map_err(|_err| de::Error::custom("failed to deserialize public key"))
        })
}

pub fn as_base64<T, S>(v: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    serializer.serialize_str(&base64::encode(v.as_ref()))
}