pub mod deserialize;
mod request;

use crate::{
    structs::question::{Attributes, Question},
    utils::{file_creation_error_messages, request_error_messages, FileCreationError},
};
use std::{
    env,
    fs::{self, File},
    io::Write,
    process::exit,
};

/// Function to clone the question in a folder given the URL.
///
/// This function will do the request to the Progression API to get question, then it creates the files
/// inside a folder named after the question title.
///
/// It will manage the errors along the way and print the error messages.
pub fn clone(url: &String, only_lang: Option<&String>) {
    let question_uri = get_question_uri_from_url(url);

    if question_uri.is_some() {
        let question = match request::http_get_question(question_uri.unwrap()) {
            Ok(question) => question,
            Err(e) => {
                request_error_messages(e);
                exit(-1)
            }
        };

        match create_files(question, only_lang) {
            Ok(_) => (),
            Err(e) => file_creation_error_messages(e),
        }
    } else {
        println!("Failed to get question URI from URL");
    }
}

/// Function to create the folder and the necessary files for a question.
///
/// This function will create the necessary files inside a folder named after the question title. If given ```Some()``` to ```only-lang```
/// it will create a question file only for the given language.
///
/// It returns a ```Result``` with either Unit or an error from the ```FileCreationError``` enum.
fn create_files(question: Question, only_lang: Option<&String>) -> Result<(), FileCreationError> {
    println!("Creating folder...");

    let titre_question = question.data.attributes.clone().unwrap().titre.unwrap();

    match fs::create_dir(&titre_question) {
        Ok(_) => {
            println!("Folder created");
            let _ = env::set_current_dir(titre_question);
        }
        Err(_) => return Err(FileCreationError::FailedCreateFolder),
    }

    println!("Creating files...");

    let prog_cli = File::create(".progcli");

    if prog_cli.is_ok() {
        let mut file = prog_cli.unwrap();
        match write!(file, "{}", question.data.id) {
            Ok(_) => println!(".progcli file created"),
            Err(_) => return Err(FileCreationError::FailedCreateDot),
        }
    } else {
        return Err(FileCreationError::FailedCreateDot);
    }

    let enonce_file = File::create("enonce.md");

    if enonce_file.is_ok() {
        let mut file = enonce_file.unwrap();
        let question_attributes = match question.data.attributes.clone() {
            Some(attributes) => attributes,
            None => return Err(FileCreationError::FailedCreateEnonce),
        };

        match write!(
            file,
            "# {}\n\n***Niveau: {}***\n\n*Par: {}*\n\n{}\n\n{}\n\n**Licence: {}**",
            question_attributes.titre.unwrap(),
            question_attributes.niveau.unwrap_or("Inconnue".to_string()),
            question_attributes
                .auteur
                .unwrap_or("Auteur inconnu".to_string()),
            question_attributes
                .description
                .unwrap_or("Aucune description".to_string()),
            question_attributes
                .énoncé
                .unwrap_or("Aucun énoncé".to_string()),
            question_attributes
                .licence
                .unwrap_or("Aucune licence".to_string())
        ) {
            Ok(_) => println!("enonce.md file created"),
            Err(_) => return Err(FileCreationError::FailedCreateEnonce),
        }
    } else {
        return Err(FileCreationError::FailedCreateEnonce);
    }

    for included_code in question.included.iter() {
        let question_code = included_code.attributes.clone();

        match only_lang {
            Some(lan) => {
                if *lan == question_code.langage.clone().unwrap() {
                    match create_question_file(question_code) {
                        Ok(_) => println!("Question file created"),
                        Err(e) => return Err(e),
                    }
                    break;
                }
            }
            None => match create_question_file(question_code) {
                Ok(_) => println!("Question file created"),
                Err(e) => return Err(e),
            },
        }
    }

    println!("File creation done!");
    Ok(())
}

/// Function to create a question file.
///
/// Since we need to create one or multiple question file under different scenario, this function
/// manages the creation of a single file. It will create a file for the given language inside
/// ```question_code``` and fill it with the given code.
fn create_question_file(question_code: Attributes) -> Result<(), FileCreationError> {
    let question_file = match question_code.description.unwrap().as_str() {
        "python" => File::create("question.py"),
        "java" => File::create("question.java"),
        "c#" => File::create("question.cs"),
        "rust" => File::create("question.rs"),
        "javascript" => File::create("question.js"),
        "kotlin" => File::create("question.kt"),
        _ => return Err(FileCreationError::FailedCreateQuestion),
    };

    if question_file.is_ok() {
        let mut file = question_file.unwrap();

        match write!(file, "{}", question_code.code.unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => return Err(FileCreationError::FailedCreateQuestion),
        }
    } else {
        return Err(FileCreationError::FailedCreateQuestion);
    }
}

/// Get the question URI from the URL.
///
/// This function will extract the question URI from a given URL.
fn get_question_uri_from_url(url: &String) -> Option<&str> {
    if url.contains("question?uri=") && url.contains("progression.crosemont.qc.ca") {
        let i = url.find("uri=").unwrap();
        Some(url.get((i + 4)..url.len()).unwrap())
    } else {
        None
    }
}
