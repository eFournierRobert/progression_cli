use std::{fs::File, io::Write, process::exit};
use crate::{request::{self, RequestError}, structs::question:: Question};

enum FileCreationError {
    FailedCreateDot,
    FailedCreateEnonce,
    FailedCreateQuestion
}

pub fn clone(url: &String, debugging: &bool) {
    let question_uri = get_question_uri_from_url(url);

    if question_uri.is_some() {
        let question = match request::http_get_question(question_uri.unwrap(), debugging) {
            Ok(question) => question,
            Err(e) => { 
                request_error_messages(e);
                exit(-1)   
            }
        };

        match create_files(question) {
            Ok(_) => (),
            Err(e) => file_creation_error_messages(e),
        }
    } else {
        println!("Failed to get question URI from URL");
    }
}

fn create_files(question: Question) -> Result<(), FileCreationError> {
    println!("Creating files...");

    let prog_cli = File::create(".progcli");

    if prog_cli.is_ok() {
        let mut file = prog_cli.unwrap();
        match write!(file, "{}", question.data.id) {
            Ok(_) => println!(".progcli file created"),
            Err(_) => return Err(FileCreationError::FailedCreateDot)
        }
    } else {
        return Err(FileCreationError::FailedCreateDot);
    }

    let enonce_file = File::create("enonce.md");

    if enonce_file.is_ok() {
        let mut file = enonce_file.unwrap();
        let question_attributes = match question.data.attributes.clone() {
            Some(attributes) => attributes,
            None => return Err(FileCreationError::FailedCreateEnonce)
        };

        match write!(
            file, "# {}\n***Niveau: {}***\n{}\n{}",
            question_attributes.titre.unwrap(),
            question_attributes.niveau.unwrap_or("Inconnue".to_string()),
            question_attributes.description.unwrap(),
            question_attributes.énoncé.unwrap_or("Aucun énoncé".to_string())
        ) {
            Ok(_) => println!("enonce.md file created"),
            Err(_) => return Err(FileCreationError::FailedCreateEnonce)
        }
    } else {
        return Err(FileCreationError::FailedCreateEnonce);
    }

    let question_code = question.included[0].attributes.clone();

    let question_file = match question_code.langage.unwrap().as_str() {
        "python" => File::create("question.py"),
        "java" => File::create("question.java"),
        "c#" => File::create("question.cs"),
        "rust" => File::create("question.rs"),
        "javascript" => File::create("question.js"),
        "kotlin" => File::create("question.kt"),
        _ => return Err(FileCreationError::FailedCreateQuestion)
    };

    if question_file.is_ok() {
        let mut file = question_file.unwrap();
        
        match write!(file, "{}", question_code.code.unwrap()) {
            Ok(_) => println!("Question file created"),
            Err(_) => return Err(FileCreationError::FailedCreateQuestion)
        }
    } else {
        return Err(FileCreationError::FailedCreateQuestion);
    }

    println!("File creation done!");
    Ok(())
}

fn get_question_uri_from_url(url: &String) -> Option<&str> {
    if url.contains("question?uri=") && url.contains("progression.crosemont.qc.ca") {
        let i = url.find("uri=").unwrap();
        Some(url.get((i + 4)..(i + 144)).unwrap())
    } else {
        None
    }
}

fn file_creation_error_messages(e: FileCreationError) {
    match e {
        FileCreationError::FailedCreateDot => {
            println!("Failed to create .progcli.");
            return
        },
        FileCreationError::FailedCreateEnonce => {
            println!("Failed to create enonce.md file.");
            return
        },
        FileCreationError::FailedCreateQuestion => {
            println!("Failed to create question file.");
            return
        }
    }
}

fn request_error_messages(e: RequestError) {
    match e {
        RequestError::AuthCreationFail => { 
            println!("Failed to create basic auth.");
            return
        },
        RequestError::QuestionDeserializeFail => {
            println!("Failed to deserialize API response.");
            return
        },
        RequestError::QuestionRequestFail => {
            println!("Failed to make HTTP request for question.");
            return;
        }
    }
}