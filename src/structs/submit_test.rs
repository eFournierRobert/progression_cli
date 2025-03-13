use serde::Serialize;

/// SubmitTest ```struct```.
/// 
/// This ```struct``` is the container for all the 
/// necessary data for a test POST request.
#[derive(Serialize)]
pub struct SubmitTest {
    pub data: Data,
}

/// Data ```struct```.
/// 
/// This ```struct``` containts some data regarding a test POST.
#[derive(Serialize)]
pub struct Data {
    #[serde(rename = "type")]
    pub data_type: String,
    pub attributes: Attributes,
}

/// Attributes ```struct```.
/// 
/// This ```struct``` contains the necessary attributes of 
/// a test POST.
#[derive(Serialize)]
pub struct Attributes {
    pub langage: String,
    pub code: String,
    pub index: String,
}
