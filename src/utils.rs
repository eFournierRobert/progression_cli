use std::{collections::HashMap, fs, io};

#[derive(Debug)]
pub enum RequestError {
    AuthCreationFail,
    QuestionRequestFail,
    QuestionDeserializeFail
}

pub fn read_uri_from_dotfile() -> String {
    fs::read_to_string(".progcli").expect(".progcli not found")
}

pub fn read_code_from_file(file: String) -> String{
    fs::read_to_string(file).expect("Couldn't read code from {file}")
}

/// A simple getter for the API URL as a String.
/// 
/// Will return a ```String``` ""https://progression.crosemont.qc.ca/api/v1//"".
pub fn get_api_url() -> String {
    String::from("https://progression.crosemont.qc.ca/api/v1//")
}

/// Ask the user for his username and password.
/// 
/// This function will prompt the user for his username and password.
/// Then stores it inside an ```Hashmap``` and the ```Hashmap``` inside an ```Result```.
pub fn get_username_password() -> Result<HashMap<String, String>, io::Error>{
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