use crate::{
    question::deserialize,
    structs::question::Question,
    utils::{RequestError, get_api_url, get_username_password},
};
use reqwest::blocking::Client;
use std::collections::HashMap;

/// Function getting the question from the API.
///
/// This function takes the question URI and makes a GET request to the API. If successful,
/// it will return a ```struct``` ```Question``` inside the ```Result```. If not, it will return
/// a ```RequestError```.
///
/// ```debugging``` makes some debugging print during execution if true.
pub fn http_get_question(question_uri: &str) -> Result<Question, RequestError> {
    let auth_result = get_username_password();
    let mut _auth = HashMap::new();

    if auth_result.is_ok() {
        _auth = auth_result.unwrap();
    } else {
        return Err(RequestError::AuthCreationFail);
    }

    let api_url = get_api_url();

    let username = _auth.get("username").unwrap();
    let password = _auth.get("password");

    println!("Fetching question...");

    let client = Client::new();
    let result = client
        .get(api_url + "question/" + question_uri + "?include=questions,ebauches,tests")
        .basic_auth(username, password)
        .send();

    if result.is_err() {
        return Err(RequestError::QuestionRequestFail);
    } else {
        let result = result.unwrap();

        if result.status() == 200 {
            match deserialize::deserialize_question(result.text().unwrap()) {
                Ok(question) => {
                    println!("Fetching done!");
                    Ok(question)
                }
                Err(_) => Err(RequestError::QuestionDeserializeFail),
            }
        } else {
            println!("{}", result.text().unwrap());
            Err(RequestError::QuestionDeserializeFail)
        }
    }
}
