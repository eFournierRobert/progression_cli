mod deserialize;
mod request;
mod serialize;

use request::post_test;
use std::{
    fs::{self, File},
    io::Write,
    process::exit,
};

use crate::{
    structs::submit_test_response::SubmitTestResponse,
    utils::{
        FileCreationError, file_creation_error_messages, get_question_file_name,
        print_submit_error_message, read_code_from_file, read_uri_from_dotfile,
        request_error_messages,
    },
};

/// Submit a test and print the response inside ```test.md```.
///
/// This function os the main function that manages submitting a test
/// and priting the result of the request inside ```test.md```.
///
/// In case of errors, it prints the error messages and exit the program.
pub fn submit_test(test_num: &String) {
    let uri = match read_uri_from_dotfile() {
        Ok(uri) => uri,
        Err(e) => {
            print_submit_error_message(e);
            exit(-1);
        }
    };
    let file = match get_question_file_name() {
        Ok(file_info) => file_info,
        Err(e) => {
            print_submit_error_message(e);
            exit(-1);
        }
    };
    let code = read_code_from_file(file.get("filename").unwrap());

    match post_test(uri, code, file.get("filetype").unwrap(), test_num) {
        Ok(test_response) => match create_test_file(test_response) {
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

/// Create the ```test.md``` file with the response of the server.
///
/// This function manages the creation of the ```test.md``` as well as
/// writing the response inside of it.
///
/// In case of errors, it will return an error from the ```FileCreationError``` enum.
fn create_test_file(test_response: SubmitTestResponse) -> Result<(), FileCreationError> {
    println!("Creating test.md");

    let test = fs::exists("test.md").unwrap();
    if test {
        _ = fs::remove_file("test.md");
    }

    let test = File::create("test.md");

    if test.is_ok() {
        let mut test_file = test.unwrap();

        match writeln!(
            test_file,
            "# Retour test\n\n***Réussi: {}***\n\nSortie Observée: \n```\n{}\n```\n\nSortie Erreur: \n```\n{}\n```\n\nFeedback: {}\n\nTemps d'exécution: {}ms",
            test_response.data.attributes.résultat,
            test_response
                .data
                .attributes
                .sortie_observée
                .unwrap_or("Aucune sortie".to_string()),
            test_response
                .data
                .attributes
                .sortie_erreur
                .unwrap_or("Aucune erreur".to_string()),
            test_response
                .data
                .attributes
                .feedback
                .unwrap_or("Aucun feedback".to_string()),
            test_response.data.attributes.temps_exécution
        ) {
            Ok(_) => {
                println!("test.md created");
                return Ok(());
            }
            Err(_) => return Err(FileCreationError::FailedToCreateTest),
        }
    } else {
        return Err(FileCreationError::FailedToCreateTest);
    }
}
