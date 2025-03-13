mod question;
pub mod structs;
mod submit_answer;
mod submit_test;
pub mod utils;

use clap::{Arg, ArgAction, command};
use submit_answer::submit_answer;

fn main() {
    let matches = command!()
        .about("Progression CLI")
        .author("Elliott Fournier-Robert")
        .arg(
            Arg::new("only-lang")
                .long("only-lang")
                .help("Clone only one language question")
                .required(false),
        )
        .arg(
            Arg::new("clone")
                .short('c')
                .long("clone")
                .help("Clone the given question with its URL")
                .required(false),
        )
        .arg(
            Arg::new("submit")
                .action(ArgAction::SetTrue)
                .long("submit")
                .short('s')
                .help("Submit the answer in the current directory")
                .required(false),
        )
        .arg(
            Arg::new("submit_test")
                .long("test")
                .short('t')
                .help("Try a specific test in enonce.md and get the response.")
                .required(false),
        )
        .arg_required_else_help(true)
        .get_matches();

    let only_lang = matches.get_one::<String>("only-lang");
    let submit = matches.get_one::<bool>("submit").unwrap();

    match matches.get_one::<String>("clone") {
        Some(url) => question::clone(url, only_lang),
        _ => {}
    }

    match matches.get_one::<String>("submit_test") {
        Some(test_num) => submit_test::submit_test(test_num),
        _ => {}
    }

    if *submit {
        submit_answer();
    }
}
