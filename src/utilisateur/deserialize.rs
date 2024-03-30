use serde::Deserialize;
use serde_json::{from_str, Error};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Token {
    data: Data
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Data {
    attributes: Attributes,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Attributes {
    username: String,
    création: i32,
    expiration: i32,
    jwt: String,
    version: String
}

pub fn deserialize_token(serialize_token: &str) -> Result<Token, Error> {
    let deserialized_token = from_str::<Token>(serialize_token);

    return deserialized_token;
}