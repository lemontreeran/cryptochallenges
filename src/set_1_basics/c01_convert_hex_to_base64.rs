use failure::Error;
use base64::STANDARD;
use base64_serde::base64_serde_type;
use serde_json; // 1.0.37
use serde::*;
use serde_derive::*;
use std::collections::BTreeMap;
use hex::FromHex;
use serde::{Serializer, Deserialize, Deserializer};
base64_serde_type!(Base64Standard, STANDARD);

#[derive(Serialize, Deserialize, Debug)]
struct Config {
  #[serde(serialize_with = "ser_base64", deserialize_with = "hex_der")]
  key: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Key {
  key: String,
}

/// Serializes `key` to a lowercase hex string.
pub fn ser_hex<T, S>(key: &T, serializer: S) -> Result<S::Ok, S::Error>
where T: AsRef<[u8]>,
      S: Serializer
{
  // serializer.serialize_str(&key.as_ref().to_hex())
  serializer.serialize_str(&hex::encode(key.as_ref()))
}

/// Deserializes a lowercase hex string to a `Vec<u8>`.
pub fn hex_der<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where D: Deserializer<'de>
{
  use serde::de::Error;
  String::deserialize(deserializer)
    .and_then(|string| Vec::from_hex(&string).map_err(|err| Error::custom(err.to_string())))
}

// fn to_base64<S>(key: &PublicKey, serializer: &mut S) -> Result<(), S::Error>
// where S: Serializer
pub fn ser_base64<T, S>(key: &T, serializer: S) -> Result<S::Ok, S::Error>
where T: AsRef<[u8]>,
      S: Serializer
{
  // serializer.serialize_str(&base64::encode(&key[..]))
  serializer.serialize_str(&base64::encode(key.as_ref()))
}

// pub fn base64_der<'de, D>(deserializer: &mut D) -> Result<Vec<u8>, D::Error>
pub fn base64_der<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where D: Deserializer<'de>
{
  use serde::de::Error;
  String::deserialize(deserializer)
    .and_then(|string| base64::decode(&string).map_err(|err| Error::custom(err.to_string())))
}

// pub fn run() -> Result<Key, Error> {
pub fn run() -> Result<(), Error> {
  let json = r###"{"key":"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"}"###;

  let config : Config = serde_json::from_str(json).unwrap();
  // let config = serde_json::from_str(json).unwrap() as Config;
  // let key = c.key;
  // println!("The config is {:?}", config);

  // let serialized = serde_json::to_string(&key).unwrap();
  let serialized = serde_json::to_string(&config).unwrap();
  // println!("serialized = {}", serialized);


  let k : Key = serde_json::from_str(&serialized).unwrap();
  println!("The key is {:?}", k.key);
  Ok(())
}
