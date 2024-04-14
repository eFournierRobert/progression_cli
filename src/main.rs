mod utilisateur;
mod question;

use clap::{command, Arg, ArgMatches, Command};

fn main() {
    let match_result = command!()
    .subcommand(
        Command::new("utilisateur")
            .subcommand(
                Command::new("connexion")
                .about("Permet de se connecter à Progression")
            )
            .subcommand(
                Command::new("deconnexion")
                .about("Permet de se déconnecter de Progression")
            )
            .about("Commandes pour gérer votre utilisateur")
            .arg_required_else_help(true)
    )
    .subcommand(
        Command::new("question")
        .arg(
            Arg::new("question_url")
            .short('r')
            .long("récupérer")
            .help("Permet de récupérer une question avec l'url")
            .aliases(["rec", "get"])
        )
        .about("Permet des actions par rapport aux questions")
        .arg_required_else_help(true)
    )
    .arg_required_else_help(true)
    .about("Progression CLI est un client en lignes de commande pour progression.dti.crosemont.quebec.")
    .version("0.0.1")
    .author("Elliott Fournier-Robert")
    .get_matches();

    match match_result.subcommand() {
        Some(("utilisateur", sous_commandes)) => {
            if sous_commandes.subcommand() != None {
                vérifier_sous_commandes_utilisateurs(sous_commandes);
            }
        },
        Some(("question", sous_commandes)) => {
            if sous_commandes.args_present() {
                vérifier_sous_commandes_question(sous_commandes);
            }
        }
        _ => {}
    }
}

fn vérifier_sous_commandes_question(sous_commandes: &ArgMatches) {
    if sous_commandes.get_one::<String>("question_url") != None {
        let token = question::récupérer_question(
            utilisateur::lire_token().as_str(),
            sous_commandes.get_one::<String>("question_url").unwrap()
        );
    }
}

fn vérifier_sous_commandes_utilisateurs(sous_commandes: &ArgMatches) {
    match sous_commandes.subcommand().unwrap().0 {
        "connexion" => utilisateur::connexion(),
        "deconnexion" => utilisateur::déconnexion(),
        _ => {}
    }
}
