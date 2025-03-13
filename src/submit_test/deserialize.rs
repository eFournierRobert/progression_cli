use serde_json::Error;

use crate::structs::submit_test_response::SubmitTestResponse;

pub fn deserialize_test(json: String) -> Result<SubmitTestResponse, Error> {
    let deser_response = serde_json::from_str(json.as_str());

    match deser_response {
        Ok(response) => Ok(response),
        Err(e) => Err(e),
    }
}
