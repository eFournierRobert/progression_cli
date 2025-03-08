use crate::structs::question::Question;
use serde_json::Error;

/// Takes a JSON ```String``` for a question and deserialize it.
/// 
/// This function takes a JSON ```String``` for a question and deserialize it
/// into a ```Question``` ```Struct```. It then stores it inside a ```Result```.
/// 
/// ```debugging``` will do a debug print if ```true```
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