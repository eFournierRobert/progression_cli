use std::io;
use base64::{prelude::BASE64_STANDARD, Engine};

const URL_API: &str = "https://progression.dti.crosemont.quebec/demo/api/v1//";

pub fn get_question(url: &String) {
    let base64_auth = match get_user_basic_auth() {
        Ok(base64_string) => base64_string,
        Err(e) => {
            println!("Error creating auth.");
            return
        }
    };
}

fn get_user_basic_auth() -> Result<String, io::Error>{
    let mut username = String::new();

    println!("Username: ");
    io::stdin().read_line(&mut username).expect("Failed to read username");

    println!("Password: ");
    let password =  match rpassword::read_password(){
        Ok(pass) => pass,
        Err(e) => return Err(e)
    };

    let encoded_auth = BASE64_STANDARD.encode(username + password.as_str());

    Ok(encoded_auth)
}