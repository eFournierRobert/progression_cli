use serde::Serialize;

#[derive(Serialize)]
pub struct SubmitTest {
    data: Data,
}

#[derive(Serialize)]
pub struct Data {
    #[serde(rename = "type")]
    data_type: String,
    attributes: Attributes,
}

#[derive(Serialize)]
pub struct Attributes {
    langage: String,
    code: String,
    index: u8,
}
