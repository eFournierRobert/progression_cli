use serde::Serialize;

/// SubmitBody ```struct```.
/// 
/// This ```struct``` is the container for all the data 
/// necessary for a answer POST request.
#[derive(Serialize)]
pub struct SubmitBody {
    pub data: Data
}

/// Data ```struct```.
/// 
/// This ```struct``` contains some data regarding a submit POST.
#[derive(Serialize)]
pub struct Data {
    #[serde(rename="type")]
    pub request_type: String,
    pub attributes: Attributes,
}

/// Attributes ```struct```.
/// 
/// This ```struct``` contains the necessary attributes of 
/// a submit POST.
#[derive(Serialize)]
pub struct Attributes {
    pub langage: String,
    pub code: String,
}