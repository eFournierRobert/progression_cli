use crate::request;

pub fn clone(url: &String) {
    request::http_get_question(url);
}