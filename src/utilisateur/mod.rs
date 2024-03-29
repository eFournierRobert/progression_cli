mod requetes;
mod deserialize;

use std::io;
use base64;
use requetes::get_token;

pub fn connexion() {
    let mut username = String::new();
    let mut mot_de_passe = String::new();
    let mut domaine = String::new();

    let stdin = io::stdin();

    print!("Nom d'utilisateur: ");
    stdin.read_line(&mut username);

    print!("\nMot de passe: ");
    stdin.read_line(&mut mot_de_passe);

    print!("\nDomaine: ");
    stdin.read_line(&mut domaine);


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
