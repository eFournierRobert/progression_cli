pub mod request;
mod question;

use clap::{command, Arg};

fn main() {
    let matches = command!()
        .about("Progression CLI")
        .author("Elliott Fournier-Robert")
        .arg(
            Arg::new("clone")
                .short('c')
                .long("clone")
                .help("Clone the given question")
                .required(false)
        )
        .arg_required_else_help(true)
        .get_matches();

    match matches.get_one::<String>("clone") {
        Some(url) => question::clone(url),
        _ => {}
    }
}
