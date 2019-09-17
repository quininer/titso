use std::error::Error;
use serde::{ Deserialize, Serialize };


#[derive(Deserialize, Serialize)]
pub struct MasterStore {
    salt: [u8; 32],
    store: [u8; 32]
}

#[derive(Deserialize, Serialize)]
pub struct Tag([u8; 16]);

#[derive(Deserialize, Serialize)]
pub struct Packet {
    nonce: [u8; 16],

    #[serde(with = "serde_bytes")]
    data: Vec<u8>,

    tag: [u8; 16]
}

#[derive(Deserialize, Serialize)]
pub struct Item {
    /// encrypted rule
    rule: Packet,
    data: Packet
}

#[derive(Deserialize, Serialize)]
pub struct Rule {
    count: u64,
    length: u16
}

#[derive(Deserialize, Serialize)]
pub struct Hint(String);
