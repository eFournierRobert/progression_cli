use crate::structs::submit_test_response::SubmitTestResponse;
use serde_json::Error;

/// Deserialize the JSON answer from the server.
///
/// This function deserialize the given JSON to a struct ```SubmitTestReponse```.
///
/// In case of error, it will return a ```serde_json::Error```.
pub fn deserialize_test(json: String) -> Result<SubmitTestResponse, Error> {
    let deser_response = serde_json::from_str(json.as_str());

    match deser_response {
        Ok(response) => Ok(response),
        Err(e) => Err(e),
    }
}
