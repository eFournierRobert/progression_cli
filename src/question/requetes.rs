use reqwest::blocking::Client;

pub fn get_question(token: &str, uri_question: &str) {
    let mut url = "https://progression.dti.crosemont.quebec/api/v1/question/".to_string();
    url.push_str(uri_question);

    let http_client = Client::new();
    let http_result = http_client
        .get(url)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer ".to_owned() + &token)
        .send();

    if http_result.is_ok() {
        println!("{}", http_result.ok().unwrap().text().unwrap());
    }else {
        panic!(
            "Erreur inconnue avec la récupération de la question: {}",
            http_result.err().unwrap()
        );
    }
}
