use reqwest::blocking::{Client, Response};

pub fn get_token(credentials: String, username: String) {
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
        .get(url)
        .header("Authorization", "Basic ".to_owned() + &credentials)
        .body(requête_body)
        .send();
}
