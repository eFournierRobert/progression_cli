use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubmitTestResponse {
    pub data: Data,
}

#[derive(Deserialize)]
pub struct Data {
    #[serde(rename = "type")]
    response_type: String,
}
#[derive(Deserialize)]
pub struct Attributes {
    pub sortie_observée: Option<String>,
    pub sortie_erreur: Option<String>,
    pub résultat: bool,
    pub feedback: Option<String>,
    pub temps_exécution: u16,
}
