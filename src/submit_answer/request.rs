use reqwest::blocking::Client;

use crate::{structs::{submit::{Attributes, Data, SubmitBody}, submit_response::SubmitResponse}, utils::{get_api_url, get_username_password, RequestError}};

use super::{deserialize::deserialize_answer, serialize::serialize_body, SubmitError};

pub fn post_answers(uri: String, code: String) -> Result<SubmitResponse, RequestError>{
    let auth = match get_username_password() {
        Ok(auth) => auth,
        Err(_) => return Err(RequestError::AuthCreationFail)
    };

    let username = auth.get("username").unwrap();
    let password = auth.get("password");
    let body = SubmitBody {
        data: Data {
            request_type: "tentative".to_string(),
            attributes: Attributes {
                code,
                langage: "python".to_string()
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