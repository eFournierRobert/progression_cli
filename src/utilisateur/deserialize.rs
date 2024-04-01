use serde::Deserialize;
use serde_json::from_str;

/// Ceci représente un token
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Token {
    /// Token doit avoir du data. Nécessaire pour la deserialization.
    data: Data
}

///Ceci représente un data dans un token
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Data {
    /// Data doit avoir des attributs. Nécessaire pour la deserialization.
    attributes: Attributes,
}

/// Ceci représente des attributs d'un token.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Attributes {
    /// Un attribut nécessaire à récupérer pour les requêtes est le nom d'utilisateur.
    username: String,
    /// Un attribut nécessaire pour communiquer avec l'API est le token JWT.
    jwt: String,
}

/// Ceci est une fonction publique qui permet de décoder le token JWT de JSON à un struct Token.
/// 
/// ## Cas d'utilisation
/// Décoder un token JWT reçu.
/// 
/// ## Paramètres
/// - serialize_token - Le JSON tu token JWT reçu par l'API de Progression.
/// 
/// ## Panic
/// S'il n'arrive pas à deserialize le JSON reçu en paramètre, 
/// il devrait arrêter la fonction et afficher un message d'erreur. 
/// La majorité du temps, l'erreur sera dû à un mauvais nom d'utilisateur ou mot de passe, 
/// donc cela est le message d'erreuré 
/// 
/// Message d'erreur : Erreur: Mauvais nom d'utilisateur ou mots de passe
/// 
/// ## Retourne
/// Une chaine de charactère avec le token et le nom d'utilisateur.
/// 
/// ## Exemple
/// ```
/// let token_json = r#"
///     {
///         "data" {
///                 "attributes" {
///                     "username": "username"
///                     "jwt" : "token jwt"  
///                 }
///             }
///     }
/// "#;
/// 
/// let deserialized_token = deserialize_token(token_json);
/// 
/// assert_eq!("jwt\nusername", deserialized_token);
/// ```
pub fn deserialize_token(serialize_token: &str) -> String {
    let deserialized_token = from_str::<Token>(serialize_token);

    if deserialized_token.is_err() {
        panic!("Erreur: Mauvais nom d'utilisateur ou mots de passe");
    }else {
        let token_unwrapped = deserialized_token.ok().unwrap();

        let mut token = token_unwrapped.data.attributes.jwt;
        token.push_str("\n");
        token.push_str(
            token_unwrapped.data.attributes.username.as_str()
        );
    
        return token;
    }
}