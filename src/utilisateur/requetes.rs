use super::deserialize;

use deserialize::deserialize_token;
use reqwest::blocking::Client;

/// Cette fonction publique permet de faire la requête HTTPS vers l'API de Pogression pour récupérer un token.
/// 
/// ## Cas d'utilisation
/// À faire pour récupérer un token de l'API de Progression. Nécessaire pour la connexion.
/// 
/// ## Paramètres
/// - credentials - Credentials en base64. Nécessaire pour le header d'autorisation "Basic".
/// - username - Le nom d'utilisateur de l'utilisateur qui veut se connecter. Nécessaire pour la requête HTTPS.
/// 
/// ## Panic
/// Panic s'il rencontre une erreur lorsqu'il fait la requête HTTPS au serveur de Progression.
/// 
/// Message d'erreur: Erreur: JWT token request: [erreur reçu de la requête HTTPS]
/// 
/// ## Retourne
/// Une chaine de charactère avec le token et le nom d'utilisateur.
/// 
/// ## Exemple
/// ```
/// let credentials = String::from("dXNlcm5hbWU6bW90X2RlX3Bhc3NlOmR0aS5jcm9zZW1vbnQucXVlYmVj");
/// let username = String::from("username"); 
/// 
/// let token = get_token(credentials, username);
/// 
/// assert_eq!("jwt\nusername", token);
/// ```
pub fn get_token(credentials: String, username: String) -> String {
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
        .body(requête_body)
        .send();

    if http_result.is_ok() {
        return deserialize_token(
            http_result.ok().unwrap().text().unwrap().as_str()
        );
    }else {
        panic!("Erreur: JWT token request: {}", http_result.err().unwrap());
    }
}