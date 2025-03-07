use std::{collections::HashMap, io};
use reqwest::blocking::Client;

pub fn http_get_question(url: &String) {
    let auth = match get_username_password() {
        Ok(auth_hasmap) => auth_hasmap,
        Err(e) => {
            println!("Error creating auth: {e}");
            return
        }
    };

    let api_url = get_api_url();

    let username = auth.get("username").unwrap();
    let password = auth.get("password");

    let client = Client::new();
    let result = client.get(
        api_url + 
        "question/" + 
        url +
        "?include=questions"
    )
            .basic_auth(
                username,
                password
            )
            .send();

    println!("{:?}", result);
}

fn get_api_url() -> String {
    String::from("https://progression.crosemont.qc.ca/api/v1//")
}

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