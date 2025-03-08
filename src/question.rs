use crate::request;

pub fn clone(url: &String) {
    let question = match request::http_get_question(url) {
        Ok(question) => question,
        Err(e) => { 
            println!("Couldn't clone question: {:?}", e);
            return
        }
    };


    println!("{:#?}", question);
}