use serde_json::Error;

use crate::structs::submit_response::SubmitResponse;

pub fn deserialize_answer(json: String) -> Result<SubmitResponse, Error> {
    let submit_response = serde_json::from_str(json.as_str());

    match submit_response {
        Ok(submit_response) => Ok(submit_response),
        Err(e) => Err(e)
    }
}