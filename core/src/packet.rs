use serde::{ Deserialize, Serialize };


#[derive(Default, Deserialize, Serialize)]
pub struct MasterSecret<'a> {
    #[serde(with = "serde_bytes")]
    pub(crate) salt: &'a [u8],

    #[serde(with = "serde_bytes")]
    pub(crate) secret: &'a [u8]
}

#[derive(Clone, Copy, Debug)]
pub struct Tag(pub [u8; 16]);

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub password: Type,

    #[serde(with = "serde_bytes")]
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
