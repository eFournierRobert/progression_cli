use serde::Deserialize;
use serde_json::from_str;

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
    jwt: String,
}

pub fn deserialize_token(serialize_token: &str) -> String {
    let deserialized_token = from_str::<Token>(serialize_token);

    if deserialized_token.is_err() {
        println!("Erreur: Mauvais nom d'utilisateur ou mots de passe");
    }

    let token_unwrapped = deserialized_token.ok().unwrap();

    let mut token = token_unwrapped.data.attributes.jwt;
    token.push_str("\n");
    token.push_str(
        token_unwrapped.data.attributes.username.as_str()
    );

    return token;
}