mod requetes;
mod deserialize;

use base64;
use requetes::get_token;

pub fn préparation_token(username: &str, mot_de_passe: &str, domaine: &str) {
    let mut credentials_préparé: String = username.to_owned();
    credentials_préparé.push_str(":");
    credentials_préparé.push_str(&mot_de_passe);
    credentials_préparé.push_str(":");
    credentials_préparé.push_str(&domaine);

    let credentials_encodés = base64::encode(credentials_préparé);

    let token = get_token(credentials_encodés, String::from(username));

    if token.is_ok() {
        println!("{:#?}", token.ok().unwrap());
    }else {
        println!("Error: Unknown error {}", token.err().unwrap());
    }
}
