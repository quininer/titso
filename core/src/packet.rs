use serde::{ Deserialize, Serialize };


#[derive(Default, Deserialize, Serialize)]
pub struct MasterSecret {
    pub(crate) salt: [u8; 32],
    pub(crate) secret: [u8; 32]
}

#[derive(Deserialize, Serialize)]
#[derive(Clone, Copy, Debug)]
pub struct Tag(pub [u8; 16]);

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub password: Type,
    pub note: Vec<u8>
}

#[derive(Deserialize, Serialize)]
pub enum Type {
    Derive(Rule),
    Fixed(String)
}

#[derive(Deserialize, Serialize)]
pub struct Rule {
    pub count: u64,
    pub length: u16,
    pub chars: Vec<char>
}
