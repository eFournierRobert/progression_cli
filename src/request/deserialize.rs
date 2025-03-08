use serde::Deserialize;
use serde_json::Error;

#[derive(Debug)]
pub struct Question {
    pub data: Data
}

#[derive(Deserialize, Debug)]
pub struct Data {
    attributes: Attributes,
    id: String,
    included: Included
}

#[derive(Deserialize, Debug)]
struct Attributes {
    auteur: String,
    description: String,
    niveau: String,
    titre: String,
    énoncé: String,
    code: String,
    langage: String
}

#[derive(Deserialize, Debug)]
struct Included {
    attributes: Attributes
}

pub fn deserialize_question(json: String) -> Result<Question, Error> {
    let data = serde_json::from_str(json.as_str());

    println!("{:?}", json.as_str());
    println!("{:?}", data);

    match data {
        Ok(data) => {
            let question: Question = Question {data: data};
            Ok(question)
        },
        Err(e) => Err(e)
    }
}