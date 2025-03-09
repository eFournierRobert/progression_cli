mod request;
mod serialize;
mod deserialize;

use std::{fs::{self, File}, io::Write, process::exit};
use crate::{structs::submit_response::SubmitResponse, utils::{file_creation_error_messages, read_code_from_file, read_uri_from_dotfile, request_error_messages, FileCreationError}};
use request::post_answers;

pub enum SubmitError {
    QuestionFileNotFound,
    CoultNotReadDirectory,
}

pub fn submit_answer() {
    let uri = read_uri_from_dotfile();
    let file_name = match get_question_file_name() {
        Ok(name) => name,
        Err(e) => {
            print_error_message(e);
            exit(-1);
        }
    };
    let code = read_code_from_file(file_name);

    match post_answers(uri, code) {
        Ok(submit_response) => {
            match create_answer_file(submit_response) {
                Ok(_) => (),
                Err(e) => {
                    file_creation_error_messages(e);
                    exit(-1);
                }
            }
        },
        Err(e) => {
            request_error_messages(e);
            exit(-1);
        }
    }
}

fn create_answer_file(submit_response: SubmitResponse) -> Result<(), FileCreationError>{
    let answer = fs::exists("answer.md").unwrap();

    if answer {
        _ =fs::remove_file("answer.md");
    }

    let answer = File::create("answer.md");

    if answer.is_ok() {
        let mut answer_file = answer.unwrap();

        match writeln!(
            answer_file,
            "# Retour\n\n***Réussi: {}***\n\nTests Réussis: {}\n\nFeedback: {}\n\n---\n\n",
            submit_response.data.attributes.réussi.unwrap(),
            submit_response.data.attributes.tests_réussis.unwrap(),
            submit_response.data.attributes.feedback.unwrap()
        ) {
            Ok(_) => {
                for res in submit_response.included {
                    match writeln!(
                        answer_file,
                        "Résultat est bon: {}\n\nSortie Observée: \n```\n{}```\n\nSortie Erreur: \n```\n{}```\n\nFeedback: {}\n\nTemps d'exécution: {}ms\n\n---",
                        res.attributes.résultat.unwrap(),
                        res.attributes.sortie_observée.unwrap_or("Aucune sortie".to_string()),
                        res.attributes.sortie_erreur.unwrap_or("Aucune erreur".to_string()),
                        res.attributes.feedback.unwrap_or("Aucun feedback".to_string()),
                        res.attributes.temps_exécution.unwrap()
                    ) {
                        Ok(_) => {},
                        Err(_) => return Err(FileCreationError::FailedCreateAnswer)
                    }
                }

                return Ok(())
            },
            Err(_) => return Err(FileCreationError::FailedCreateAnswer)
        }
    } else {
        return Err(FileCreationError::FailedCreateAnswer)
    }
}

fn get_question_file_name() -> Result<String, SubmitError>{
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        match path {
            Ok(res) => {
                let file_name = res.file_name();

                match file_name.to_string_lossy().get(0..8) {
                    Some(x) => { 
                        if x == "question".to_string() {
                            return Ok(file_name.to_string_lossy().to_string());
                        }
                    },
                    None => {}
                }
            },
            Err(_) => return Err(SubmitError::CoultNotReadDirectory)
        }
    }

    Err(SubmitError::QuestionFileNotFound)
}

fn print_error_message(e: SubmitError) {
    match e {
        SubmitError::QuestionFileNotFound => {
            println!("Could not find Question file");
            return;
        }
        SubmitError::CoultNotReadDirectory => {
            println!("Could not read current directory");
            return;
        }
    }
}