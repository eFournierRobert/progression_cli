use reqwest::Client;

use crate::utils::{get_api_url, get_username_password, RequestError};

// pub fn post_answers(uri: String, code: String) -> Result<(), RequestError>{
//     let auth = match get_username_password() {
//         Ok(auth) => auth,
//         Err(_) => return Err(RequestError::AuthCreationFail)
//     };

//     let api_url = get_api_url();
//     let username = auth.get("username").unwrap();
//     let password = auth.get("password");

//     let client = Client::new();
//     let result = client.post()
// }