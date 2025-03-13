use reqwest::blocking::Client;

use crate::{
    structs::{
        submit_test::{Attributes, Data, SubmitTest},
        submit_test_response::SubmitTestResponse,
    },
    submit_test::{deserialize::deserialize_test, serialize::serialize_body},
    utils::{RequestError, get_api_url, get_langage, get_username_password},
};

/// Post the given test for a given question and receive a ```SubmitTestResponse``` for it.
///
/// This function submits an answer for the given question and returns a struct ``SubmitTestResponse```
/// with the whole response from the server.
///
/// In case of error, it will return an error from the ```RequestError``` enum.
pub fn post_test(
    uri: String,
    code: String,
    file_type: &String,
    test_num: &String,
) -> Result<SubmitTestResponse, RequestError> {
    let auth = match get_username_password() {
        Ok(auth) => auth,
        Err(_) => return Err(RequestError::AuthCreationFail),
    };

    println!("Sending request for test {}...", test_num);

    let language = match get_langage(&file_type) {
        Some(lan) => lan,
        None => return Err(RequestError::FailToGetLangage),
    };

    let username = auth.get("username").unwrap();
    let password = auth.get("password");
    let body = SubmitTest {
        data: Data {
            data_type: "resultat".to_string(),
            attributes: Attributes {
                langage: language,
                code,
                index: test_num.to_owned(),
            },
        },
    };

    let url = get_api_url() + "question/" + &uri + "/resultats";
    let body = match serialize_body(body) {
        Ok(b) => b,
        Err(_) => return Err(RequestError::SubmitSerializeFail),
    };

    let client = Client::new();
    let result = client
        .post(url)
        .basic_auth(username, password)
        .header("Content-Type", "application/json")
        .body(body)
        .send();

    if result.is_err() {
        return Err(RequestError::SubmitRequestFail);
    } else {
        println!("Received result");

        let result = result.unwrap();

        if result.status() == 200 {
            match deserialize_test(result.text().unwrap()) {
                Ok(test_response) => Ok(test_response),
                Err(_) => Err(RequestError::SubmitDeserializeFail),
            }
        } else {
            println!("{}", result.text().unwrap());
            return Err(RequestError::SubmitRequestFail);
        }
    }
}
