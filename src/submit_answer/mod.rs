mod deserialize;
mod request;
mod serialize;

use crate::{
    structs::submit_response::SubmitResponse,
    utils::{
        FileCreationError, file_creation_error_messages, get_question_file_name,
        print_submit_error_message, read_code_from_file, read_uri_from_dotfile,
        request_error_messages,
    },
};
use request::post_answers;
use std::{
    fs::{self, File},
    io::Write,
    process::exit,
};

/// Submit an answer and print the response inside ```answer.md```.
///
/// This function is the main function that manages submitting an answer
/// and printing the result inside the file ```answer.md```.
///
/// In case of errors, it prints the error messages and exit the program.
pub fn submit_answer() {
    let uri = read_uri_from_dotfile();
    let file = match get_question_file_name() {
        Ok(file_info) => file_info,
        Err(e) => {
            print_submit_error_message(e);
            exit(-1);
        }
    };
    let code = read_code_from_file(file.get("filename").unwrap());

    match post_answers(uri, code, file.get("filetype").unwrap()) {
        Ok(submit_response) => match create_answer_file(submit_response) {
            Ok(_) => (),
            Err(e) => {
                file_creation_error_messages(e);
                exit(-1);
            }
        },
        Err(e) => {
            request_error_messages(e);
            exit(-1);
        }
    }
}

/// Create the ```answer.md``` file with the response of the server.
///
/// This function manages the creation of the ```answer.md``` as well as writing
/// the response inside of it.
///
/// In case of errors, it will return an error from the ```FileCreationError``` enum.
fn create_answer_file(submit_response: SubmitResponse) -> Result<(), FileCreationError> {
    println!("Creating answer.md...");

    let answer = fs::exists("answer.md").unwrap();

    if answer {
        _ = fs::remove_file("answer.md");
    }

    let answer = File::create("answer.md");

    if answer.is_ok() {
        let mut answer_file = answer.unwrap();

        match writeln!(
            answer_file,
            "# Retour\n\n***Réussi: {}***\n\nTests Réussis: {}\n\nFeedback: {}\n\n---\n\n",
            submit_response.data.attributes.réussi.unwrap(),
            submit_response.data.attributes.tests_réussis.unwrap(),
            submit_response
                .data
                .attributes
                .feedback
                .unwrap_or("Aucun feedback".to_string())
        ) {
            Ok(_) => {
                for res in submit_response.included {
                    match writeln!(
                        answer_file,
                        "Résultat est bon: {}\n\nSortie Observée: \n```\n{}\n```\n\nSortie Erreur: \n```\n{}\n```\n\nFeedback: {}\n\nTemps d'exécution: {}ms\n\n---",
                        res.included_attrbiutes.résultat.unwrap(),
                        res.included_attrbiutes
                            .sortie_observée
                            .unwrap_or("Aucune sortie".to_string()),
                        res.included_attrbiutes
                            .sortie_erreur
                            .unwrap_or("Aucune erreur".to_string()),
                        res.included_attrbiutes
                            .feedback
                            .unwrap_or("Aucun feedback".to_string()),
                        res.included_attrbiutes.temps_exécution.unwrap()
                    ) {
                        Ok(_) => {}
                        Err(_) => return Err(FileCreationError::FailedCreateAnswer),
                    }
                }

                println!("answer.md created");
                return Ok(());
            }
            Err(_) => return Err(FileCreationError::FailedCreateAnswer),
        }
    } else {
        return Err(FileCreationError::FailedCreateAnswer);
    }
}
