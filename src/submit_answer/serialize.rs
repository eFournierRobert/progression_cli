use crate::structs::submit::SubmitBody;
use serde_json::Error;

/// Serialize the given ```SubmitBody``` to JSON.
///
/// Function that serialize the given ```SubmitBody``` to JSON.
///
/// In case of error, it will return a ```serde_json::Error```.
pub fn serialize_body(body: SubmitBody) -> Result<String, Error> {
    let ser_body = serde_json::to_string_pretty(&body);

    match ser_body {
        Ok(body) => Ok(body),
        Err(e) => Err(e),
    }
}
