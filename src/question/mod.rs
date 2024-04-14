mod requetes;

use requetes::get_question;

pub fn récupérer_question(token_et_username: &str, url_question: &str) {
    let token = récupérer_token(token_et_username);
    let uri = récupérer_uri(url_question);

    get_question(token, uri);
}

fn récupérer_uri(url: &str) -> &str {
    return url.split("uri=").collect::<Vec<_>>()[1];
}

fn récupérer_token(token_et_username: &str) -> &str {
    return token_et_username.split("\n").collect::<Vec<_>>()[0];
}

fn récupérer_username(token_et_username: &str) -> &str {
    return token_et_username.split("\n").collect::<Vec<_>>()[1];
}
