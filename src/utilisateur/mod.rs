mod requetes;

use base64;
use requetes::get_token;

pub fn préparation_token(username: String, mot_de_passe: String, domaine: String) {
    let mut credentials_préparé: String = username.to_owned();
    credentials_préparé.push_str(":");
    credentials_préparé.push_str(&mot_de_passe);
    credentials_préparé.push_str(":");
    credentials_préparé.push_str(&domaine);

    let credentials_encodés = base64::encode(credentials_préparé);

    get_token(credentials_encodés, username);
}
