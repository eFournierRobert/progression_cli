use std::{collections::HashMap, io};
use reqwest::blocking::Client;
use crate::{question::deserialize, structs::question::Question};

#[derive(Debug)]
pub enum RequestError {
    AuthCreationFail,
    QuestionRequestFail,
    QuestionDeserializeFail
}

/// Function getting the question from the API.
/// 
/// This function takes the question URI and makes a GET request to the API. If successful,
/// it will return a ```struct``` ```Question``` inside the ```Result```. If not, it will return
/// a ```RequestError```.
/// 
/// ```debugging``` makes some debugging print during execution if true.
pub fn http_get_question(question_uri: &str, debugging: &bool) -> Result<Question, RequestError>{
    let auth_result = get_username_password();
    let mut _auth = HashMap::new();

    if auth_result.is_ok() {
        _auth = auth_result.unwrap();
    } else {
        if *debugging {
            println!("{:?}", auth_result.err());
        }
        return Err(RequestError::AuthCreationFail);
    }

    let api_url = get_api_url();

    let username = _auth.get("username").unwrap();
    let password = _auth.get("password");

    println!("Fetching question...");

    let client = Client::new();
    let result = client.get(
        api_url + 
        "question/" + 
        question_uri +
        "?include=questions,ebauches"
    )
            .basic_auth(
                username,
                password
            )
            .send();

    if result.is_err() {
        if *debugging {
            println!("{:?}", result.err());
        }
        return Err(RequestError::QuestionRequestFail);
    } else {
        let result = result.unwrap();

        if result.status() == 200 {
            match deserialize::deserialize_question(result.text().unwrap(), debugging) {
                Ok(question) => {
                    println!("Fetching done!");
                    Ok(question)
                },
                Err(e) => {
                    if *debugging {
                        println!("{:?}", e);
                    }
                    Err(RequestError::QuestionDeserializeFail)
                }
            }
        } else {
            println!("{}", result.text().unwrap());
            Err(RequestError::QuestionDeserializeFail)
        }
        
    }
}

/// A simple getter for the API URL as a String.
/// 
/// Will return a ```String``` ""https://progression.crosemont.qc.ca/api/v1//"".
fn get_api_url() -> String {
    String::from("https://progression.crosemont.qc.ca/api/v1//")
}

/// Ask the user for his username and password.
/// 
/// This function will prompt the user for his username and password.
/// Then stores it inside an ```Hashmap``` and the ```Hashmap``` inside an ```Result```.
fn get_username_password() -> Result<HashMap<String, String>, io::Error>{
    let mut username = String::new();

    println!("Username: ");
    io::stdin().read_line(&mut username).expect("Failed to read username");
    username.pop(); // Remove newline character

    println!("Password: ");
    let password =  match rpassword::read_password(){
        Ok(pass) => pass,
        Err(e) => return Err(e)
    };

    let mut hashmap = HashMap::new();
    hashmap.insert(String::from("username"), username.clone());
    hashmap.insert(String::from("password"), password);

    Ok(hashmap)
}