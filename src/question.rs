use std::{error::Error, fs::File, io::{self, Write}, process::exit};
use crate::{request::{self, RequestError}, structs::question::{self, Question}};

pub fn clone(url: &String) {
    let question_uri = get_question_uri_from_url(url);

    if question_uri.is_some() {
        let question = match request::http_get_question(question_uri.unwrap()) {
            Ok(question) => question,
            Err(e) => { 
                request_error_messages(e);
                exit(-1)   
            }
        };

        println!("Fetching done!");

        create_files(question);

        println!("File creation done!");
    } else {
        println!("Failed to get question URI from URL");
    }
}

fn create_files(question: Question) {
    println!("Creating files...");
    let mut prog_cli = File::create(".progcli");
    let prog_cli_result = write!(prog_cli.unwrap(), "{}", question.data.id.clone().unwrap());

    if prog_cli_result.is_err() {
        println!("Failed to create .progcli file.");
        exit(-1);
    }

    let mut md_file = File::create("enonce.md").unwrap();
    writeln!(md_file, "# {}", question.data.attributes.clone().unwrap().titre.unwrap());
    //writeln!(md_file, "*Par {}*", question.data.attributes.clone().unwrap().auteur.unwrap());
    writeln!(md_file, "{}", question.data.attributes.clone().unwrap().description.unwrap());

    let mut file = match question.included[0].attributes.langage.clone().unwrap().as_str() {
        "python" => File::create("question.py"),
        "java" => File::create("question.java"),
        "c#" => File::create("question.cs"),
        "rust" => File::create("question.rs"),
        "javascript" => File::create("question.js"),
        _ => {
            println!("Failed to create file or unsupported language.");
            exit(-1);
        }
    };

    if file.is_ok() {
        let question_result = write!(file.unwrap(), "{}", question.included[0].attributes.code.clone().unwrap());

        if question_result.is_err() {
            println!("Failed to create question file.");
            exit(-1);
        }
    }
}

fn get_question_uri_from_url(url: &String) -> Option<&str> {
    if url.contains("question?uri=") && url.contains("progression.crosemont.qc.ca") {
        let i = url.find("uri=").unwrap();
        Some(url.get((i + 4)..(i + 144)).unwrap())
    } else {
        None
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