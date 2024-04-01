extern crate rpassword;

mod requetes;
mod deserialize;

use std::{fs, io, path::PathBuf};
use requetes::get_token;
use rpassword::read_password;
use simple_home_dir::home_dir;

/// Ceci est une fonction publique qui récupère les identifiants de l'utilisateur et 
/// qui fait les étapes nécessaires à la connexion à Progression.
/// 
/// ## Cas d'utilisation
/// Commence toute les étape nécessaire à la connexion à Progression.
/// 
/// ## Retourne 
/// Unit
pub fn connexion() {
    let mut username = String::new();

    let stdin = io::stdin();

    println!("Nom d'utilisateur: ");
    _ = stdin.read_line(&mut username);

    println!("\nMot de passe: ");
    let mot_de_passe = read_password().unwrap();

    let credentials_encodés = préparation_credentials(
        &mut username,
        mot_de_passe.as_str()
    );

    let token = get_token(credentials_encodés, username.to_string());

    stocker_token(token.as_str());
}

/// Ceci est une fonction publique qui supprime le contenu du fichier contenant le token et le nom d'utilisateur.
/// 
/// ## Cas d'utilisation
/// Pour supprimer le token de l'utilisateur, donc le "déconnecter" de Progression.
/// 
/// ## Retourne 
/// Unit
pub fn déconnexion() {
    let path_token = get_fichier_token();

    _ = fs::write(path_token, "");

    println!("Déconnexion réussi");
}

/// Ceci est une fonction qui permet de bien formatter et d'encoder le nom d'utilisateur,
/// le mot de passe et le domaine en base64.
/// 
/// ## Cas d'utilisation
/// Pour faire le header d'autorisation "Basic" qui est requis pour l'authentification initiale à Progression.
/// 
/// ## Paramètres
/// - username - Le nom d'utilisateur de l'usager.
/// - mot_de_passe - Le mot de passe de l'usager.
/// 
/// ## Retourne 
/// Nom d'utilisateur, mot de passe et domaine en base64.
/// 
/// ## Exemple
/// ```
/// let username = "username";
/// let mot_de_passe = "mot_de_passe";
/// 
/// let credentials_encodés = préparation_credentials(username, mot_de_passe);
/// 
/// assert_eq!(
///     "dXNlcm5hbWU6bW90X2RlX3Bhc3NlOmR0aS5jcm9zZW1vbnQucXVlYmVj",
///     credentials_encodé
/// );
/// ```
fn préparation_credentials(username: &mut String, mot_de_passe: &str) -> String {
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

/// Ceci est une fonction qui permet de stocker le token JWT et le nom d'utilisateur dans un fichier.
/// 
/// ## Cas d'utilisation
/// Pour garder en mémoire le token JWT et le nom d'utilisateur de l'usager qui se connecte.
/// **Token JWT est nécessaire à toute les requêtes**
/// 
/// ## Paramètres 
/// - token - Le token et le nom d'utilisateur à stocker dans un fichier.
/// 
/// ## Panics
/// Panic lorsqu'il n'arrive pas à créer le dossier qui doit contenir le fichier contenant le token JWT.
/// Message d'erreur : Erreur: Impossible d'enregistrer le token d'authentification \n [erreur reçu lors de la création du dossier]
/// 
/// ## Retourne 
/// Unit
fn stocker_token(token: &str) {
    let path = get_dossier_progression();
    let data_path = std::path::Path::new(path.as_os_str());

    if !data_path.exists() {
        let résultat_création = fs::create_dir(&path);

        if résultat_création.is_err() {
            panic!(
                "Erreur: Impossible d'enregistrer le token d'authentification\n{}",
                résultat_création.err().unwrap()
            );
        }
    }

    let path_token = get_fichier_token();

    _ = fs::write(path_token, token);
}

/// Ceci est une fonction qui permet de récupérer le dossier principale de progression_cli.
/// 
/// Le dossier est stocké dans : ~/.config/progressioncli
/// 
/// ## Retourne 
/// Le path du dossier progressioncli.
fn get_dossier_progression() -> PathBuf {
    let mut path = home_dir().unwrap();
    path.push(".config/progressioncli");

    return path;
}

/// Ceci est une fonction qui permet de récupérer le fichier du token.
/// 
/// Le fichier est stocké dans : ~/.config/progressioncli/token
/// 
/// ## Retourne 
/// Le path du fichier token.
fn get_fichier_token() -> PathBuf{
    let mut path = home_dir().unwrap();
    path.push(".config/progressioncli");
    path.push("token");

    return path;
}