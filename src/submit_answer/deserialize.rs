use serde_json::Error;
use crate::structs::submit_response::SubmitResponse;

/// Deserialize the JSON answer from the server.
/// 
/// This function deserialized the given JSON to a struct ```SubmitResponse```.
/// 
/// In case of error, it will return a ```serde_json::Error```.
pub fn deserialize_answer(json: String) -> Result<SubmitResponse, Error> {
    let submit_response = serde_json::from_str(json.as_str());

    match submit_response {
        Ok(submit_response) => Ok(submit_response),
        Err(e) => Err(e)
    }
}