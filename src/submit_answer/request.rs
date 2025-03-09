use reqwest::blocking::Client;
use crate::{structs::{submit::{Attributes, Data, SubmitBody}, submit_response::SubmitResponse}, utils::{get_api_url, get_username_password, RequestError}};
use super::{deserialize::deserialize_answer, serialize::serialize_body};

/// Post an answer for a given question and receive a ```SubmitResponse``` for it.
/// 
/// This function submits an answer for the given question and returns a struct ```SubmitResponse```
/// with the whole response from the server.
/// 
/// In case of error, it will return an error from the ```RequestError``` enum.
pub fn post_answers(uri: String, code: String, file_type: &String) -> Result<SubmitResponse, RequestError>{
    let auth = match get_username_password() {
        Ok(auth) => auth,
        Err(_) => return Err(RequestError::AuthCreationFail)
    };

    println!("Sending answer...");

    let language = match get_langage(file_type) {
        Some(lan) => lan,
        None => return Err(RequestError::FailToGetLangage)
    };

    let username = auth.get("username").unwrap();
    let password = auth.get("password");
    let body = SubmitBody {
        data: Data {
            request_type: "tentative".to_string(),
            attributes: Attributes {
                code,
                langage: language
            }
        }
    };
    let url = get_api_url() + 
        "avancement/" + 
        &username + 
        "/" + 
        &uri +
        "/tentatives?include=resultats"; 
    let body = match serialize_body(body) {
        Ok(b) => b,
        Err(_) => return Err(RequestError::SubmitSerializeFail)
    };

    let client = Client::new();
    let result = client.post(url)
        .basic_auth(
            username, 
            password
        )
        .header("Content-Type", "application/json")
        . body(body)
        .send();

    if result.is_err() {
        return Err(RequestError::SubmitRequestFail)
    } else {
        println!("Received result");

        let result = result.unwrap();

        if result.status() == 200 {
            match deserialize_answer(result.text().unwrap()) {
                Ok(submit_response) => Ok(submit_response),
                Err(_) => Err(RequestError::SubmitDeserializeFail)
            }
        } else {
            println!("{}", result.text().unwrap());
            return Err(RequestError::SubmitRequestFail)
        }
    }
}

/// Returns the language for the given file extension.
/// 
/// This function takes a file extension and returns the 
/// programming language for that file extension in an ```Option```.
fn get_langage(file_type: &String) -> Option<String> {
    return match file_type.as_str() {
        ".py" => Some(String::from("python")),
        ".java" => Some(String::from("java")),
        ".cs" => Some(String::from("c#")),
        ".rs" => Some(String::from("rust")),
        ".js" => Some(String::from("javascript")),
        ".kt" => Some(String::from("kotlin")),
        _ => None
    };
}