extern crate rpassword;

mod requetes;
mod deserialize;

use std::{fs, io};
use requetes::get_token;
use rpassword::read_password;

pub fn connexion() {
    let mut username = String::new();

    let stdin = io::stdin();

    println!("Nom d'utilisateur: ");
    _ = stdin.read_line(&mut username);

    println!("\nMot de passe: ");
    let mot_de_passe = read_password().unwrap();

    let credentials_encodés = préparation_credentials(username.clone(), mot_de_passe);

    let token = get_token(credentials_encodés, username.to_string());

    stocker_token(token.as_str());
}

pub fn déconnexion() {
    _ = fs::write("./progressioncli/token", "");

    println!("Déconnexion réussi");
}

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

fn stocker_token(token: &str) {
    let path = "./progressioncli";
    let data_path = std::path::Path::new(path);

    if !data_path.exists() {
        let résultat_création = fs::create_dir(path);

        if résultat_création.is_err() {
            panic!(
                "Erreur: Impossible d'enregistrer le token d'authentification\n{}",
                résultat_création.err().unwrap()
            );
        }
    }

    let token_path = "./progressioncli/token";

    _ = fs::write(token_path, token);
}