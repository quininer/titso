use serde::{ Deserialize, Serialize };


#[derive(Deserialize, Serialize)]
pub struct MasterStore {
    salt: [u8; 32],
    store: [u8; 32]
}

#[derive(Deserialize, Serialize)]
#[derive(Clone, Copy, Debug)]
pub struct Tag([u8; 16]);

#[derive(Deserialize, Serialize)]
pub struct Packet {
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,

    tag: [u8; 16]
}

#[derive(Deserialize, Serialize)]
pub struct Item {
    password: Option<Type>,
    data: String
}

#[derive(Deserialize, Serialize)]
pub enum Type {
    Derive(Rule),
    Fixed(Vec<u8>)
}

#[derive(Deserialize, Serialize)]
pub struct Rule {
    count: u64,
    length: u16
}

#[derive(Deserialize, Serialize)]
pub struct Hint(String);
