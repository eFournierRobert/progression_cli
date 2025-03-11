use crate::structs::question::Question;
use serde_json::Error;

/// Takes a JSON ```String``` for a question and deserialize it.
///
/// This function takes a JSON ```String``` for a question and deserialize it
/// into a ```Question``` ```Struct```. It then stores it inside a ```Result```.
///
/// ```debugging``` will do a debug print if ```true```
pub fn deserialize_question(json: String) -> Result<Question, Error> {
    let question = serde_json::from_str(json.as_str());

    match question {
        Ok(question) => Ok(question),
        Err(e) => Err(e),
    }
}
