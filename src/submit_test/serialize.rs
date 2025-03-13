use serde_json::Error;
use crate::structs::submit_test::SubmitTest;

/// Serialize the given ```SubmitTest``` to JSON.
/// 
/// Function that serialize the given ```SubmitTest``` to JSON.
/// 
/// In case of error, it will return a ```serde_json::Error```.
pub fn serialize_body(body: SubmitTest) -> Result<String, Error> {
    let ser_body = serde_json::to_string_pretty(&body);

    match ser_body {
        Ok(body) => Ok(body),
        Err(e) => Err(e),
    }
}
