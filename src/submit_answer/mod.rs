mod request;
mod serialize;
mod deserialize;

use std::{fs, process::exit};

use crate::{structs::submit_response, utils::{read_code_from_file, read_uri_from_dotfile, request_error_messages}};
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
        Ok(submit_response) => println!("{:?}", submit_response),
        Err(e) => {
            request_error_messages(e);
            exit(-1);
        }
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