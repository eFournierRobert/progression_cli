mod utilisateur;

use clap::{command, Arg, Command};

fn main() {
    let match_result = command!()
    .subcommand(
        Command::new("utilisateur")
            .arg(
                Arg::new("connecter")
                    .short('c')
                    .long("connecter")
                    .aliases(["conn", "login"])
                    .help("Permet de se connecter à Progression.")
            )
            .arg(
                Arg::new("déconnecter")
                    .short('d')
                    .long("deconnecter")
                    .aliases(["déconnecter", "logout", "deconn"])
                    .help("Permet de se déconnecter de Progression.")
            )
            .about("Commandes pour gérer votre utilisateur")
    )
    .about("Progression CLI est un client en ligne de commande pour une instance de Progression.")
    .version("0.0.1")
    .get_matches();
}
