use serde::Serialize;

#[derive(Serialize)]
pub struct SubmitBody {
    pub data: Data
}

#[derive(Serialize)]
pub struct Data {
    #[serde(rename="type")]
    pub request_type: String,
    pub attributes: Attributes,
}

#[derive(Serialize)]
pub struct Attributes {
    pub langage: String,
    pub code: String,
}