use serde::Serialize;

#[derive(Serialize)]
pub struct SubmitTest {
    pub data: Data,
}

#[derive(Serialize)]
pub struct Data {
    #[serde(rename = "type")]
    pub data_type: String,
    pub attributes: Attributes,
}

#[derive(Serialize)]
pub struct Attributes {
    pub langage: String,
    pub code: String,
    pub index: String,
}
