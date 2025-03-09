use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SubmitResponse {
    pub data: Data,
    pub included: Vec<Included>
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub attributes: Attributes
}

#[derive(Deserialize, Debug)]
pub struct Attributes {
    pub feedback: Option<String>,
    pub réussi: Option<bool>,
    pub tests_réussis: Option<u8>,
    pub résultat: Option<bool>,
    pub sortie_erreur: Option<String>,
    pub sortie_observée: Option<String>,
    pub temps_exécution: Option<u16>
}

#[derive(Deserialize, Debug)]
pub struct Included {
    pub attributes: Attributes
}