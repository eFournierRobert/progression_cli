use super::deserialize;

use deserialize::{deserialize_token, Token};
use serde_json::Error;
use reqwest::blocking::Client;

pub fn get_token(credentials: String, username: String) -> Result<Token, Error> {
    let mut url = "https://progression.dti.crosemont.quebec/api/v1/user/".to_string();
    url.push_str(&username);
    url.push_str("/tokens");

    let requête_body = r#"
    {
        "data": {
            "expiration": "+300",
            "fingerprint": false,
            "ressources": {
                "api": {
                    "url": "^(users?|ebauche|question|test|avancement|tentative|commentaire|test|sauvegarde)((?!tokens|cles).)*/?$",
                    "method": "^.*"
                }
            }
        }
    }
    "#;

    let http_client = Client::new();
    let http_result = http_client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", "Basic ".to_owned() + &credentials)
        .body(requête_body.to_owned())
        .send();

    if http_result.is_ok() {
        return deserialize_token(
            http_result.ok().unwrap().text().unwrap().as_str()
        );
    }else {
        panic!("Error: JWT token request: {}", http_result.err().unwrap());
    }
}