use serde::{ Deserialize, Serialize };


#[derive(Deserialize, Serialize)]
pub struct MasterSecret {
    pub salt: [u8; 32],
    pub secret: [u8; 32]
}

#[derive(Deserialize, Serialize)]
#[derive(Clone, Copy, Debug)]
pub struct Tag(pub [u8; 16]);

#[derive(Deserialize, Serialize)]
pub struct Packet {
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,

    pub tag: [u8; 16]
}

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub password: Type,
    pub data: String
}

#[derive(Deserialize, Serialize)]
pub enum Type {
    Derive(Rule),
    Fixed(Vec<u8>)
}

#[derive(Deserialize, Serialize)]
pub struct Rule {
    pub count: u64,
    pub length: u16
}

#[derive(Deserialize, Serialize)]
pub struct Hint(pub String);
