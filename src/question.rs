use std::process::exit;

use crate::request::{self, RequestError};

pub fn clone(url: &String) {
    let question_uri = get_question_uri_from_url(url);

    if question_uri.is_some() {
        let question = match request::http_get_question(question_uri.unwrap()) {
            Ok(question) => question,
            Err(e) => { 
                request_error_messages(e);
                exit(1)   
            }
        };

        println!("{:#?}", question);
    } else {
        println!("Failed to get question URI from URL");
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