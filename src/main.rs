pub mod request;
pub mod structs;
mod question;

use clap::{command, Arg, ArgAction};

fn main() {
    let matches = command!()
        .about("Progression CLI")
        .author("Elliott Fournier-Robert")
        .arg(
            Arg::new("debugging")
                .action(ArgAction::SetTrue)
                .long("debug")
                .help("Print to STDOUT the full errors")
                .required(false)
        )
        .arg(
            Arg::new("only-lang")
                .long("only-lang")
                .help("Clone only one language question")
                .required(false)
        )
        .arg(
            Arg::new("clone")
                .short('c')
                .long("clone")
                .help("Clone the given question with its URL")
                .required(false)
        )
        .arg_required_else_help(true)
        .get_matches();

    let debugging = matches.get_one::<bool>("debugging").unwrap();
    let only_lang = matches.get_one::<String>("only-lang");

    match matches.get_one::<String>("clone") {
        Some(url) => question::clone(url, debugging, only_lang),
        _ => {}
    }
}
