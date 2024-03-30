mod utilisateur;

use clap::{command, ArgMatches, Command};

fn main() {
    let match_result = command!()
    .subcommand(
        Command::new("utilisateur")
            .subcommand(
                Command::new("connexion")
            )
            .subcommand(
                Command::new("deconnexion")
            )
            .about("Commandes pour gérer votre utilisateur")
            .arg_required_else_help(true)
    )
    .arg_required_else_help(true)
    .about("Progression CLI est un client en lignes de commande pour Progression.")
    .version("0.0.1")
    .author("Elliott Fournier-Robert")
    .get_matches();

    match match_result.subcommand() {
        Some(("utilisateur", sous_commandes)) => {
            if sous_commandes.subcommand() != None {
                vérifier_sous_commandes_utilisateurs(sous_commandes);
            }
        },
        _ => {println!("womp womp");}
    }
}

fn vérifier_sous_commandes_utilisateurs(sous_commandes: &ArgMatches) {
    match sous_commandes.subcommand().unwrap().0 {
        "connexion" => utilisateur::connexion(),
        "deconnexion" => println!("PAS IMPLÉMENTÉ"),
        _ => {}
    }
}