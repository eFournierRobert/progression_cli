use crate::structs::question::Question;
use serde_json::Error;

pub fn deserialize_question(json: String, debugging: &bool) -> Result<Question, Error> {
    let question = serde_json::from_str(json.as_str());

    if *debugging {
        println!("{:?}", json);
    }

    match question {
        Ok(question) => Ok(question),
        Err(e) => Err(e)
    }
}