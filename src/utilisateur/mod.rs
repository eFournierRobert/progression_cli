extern crate rpassword;

mod requetes;
mod deserialize;

use std::io;
use base64;
use requetes::get_token;
use rpassword::read_password;

fn préparation_credentials(mut username: String, mot_de_passe: String) -> String {
    let len_username = username.len();
    
    username.truncate(len_username - 1);

    let mut credentials_préparé: String = username.to_owned();
    credentials_préparé.push_str(":");
    credentials_préparé.push_str(&mot_de_passe);
    credentials_préparé.push_str(":");
    credentials_préparé.push_str("dti.crosemont.quebec");

    let credentials_encodés = base64::encode(credentials_préparé);

    return credentials_encodés;
}

pub fn connexion() {
    let mut username = String::new();

    let stdin = io::stdin();

    println!("Nom d'utilisateur: ");
    _ = stdin.read_line(&mut username);

    println!("\nMot de passe: ");
    let mot_de_passe = read_password().unwrap();

    let credentials_encodés = préparation_credentials(username.clone(), mot_de_passe);

    let token = get_token(credentials_encodés, username.to_string());

    if token.is_ok() {
        println!("{:#?}", token.ok().unwrap());
    }else {
        println!("Erreur: Mauvais nom d'utilisateur ou mot de passe");
    }
}
