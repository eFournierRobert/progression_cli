use crate::structs::question::Question;
use serde_json::Error;

pub fn deserialize_question(json: String) -> Result<Question, Error> {
    let question = serde_json::from_str(json.as_str());

    match question {
        Ok(question) => Ok(question),
        Err(e) => Err(e)
    }
}