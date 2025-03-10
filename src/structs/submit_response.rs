use serde::Deserialize;

/// SubmitResponse ```struct```.
/// 
/// This ```struct``` is the container for all the data received
/// from a submit POST.
#[derive(Deserialize, Debug)]
pub struct SubmitResponse {
    pub data: Data,
    pub included: Vec<Included>
}

/// /// Data ```struct```.
/// 
/// This ```struct``` contains some data regarding a 
/// submit response.
#[derive(Deserialize, Debug)]
pub struct Data {
    pub attributes: Attributes
}

/// Attributes ```struct```.
/// 
/// This ```struct``` contains all the attributes received
/// after a submit POST.
#[derive(Deserialize, Debug)]
pub struct Attributes {
    pub feedback: Option<String>,
    pub réussi: Option<bool>,
    pub tests_réussis: Option<u8>
}

/// IncludedAttributes ```struct```
/// 
/// This ```struct``` contains all the attributes included
/// in the response of a submit POST.
#[derive(Deserialize, Debug)]
pub struct IncludedAttributes {
    pub feedback: Option<String>,
    pub résultat: Option<bool>,
    pub sortie_erreur: Option<String>,
    pub sortie_observée: Option<String>,
    pub temps_exécution: Option<u16>
}

/// Included ```struct```.
/// 
/// This ```struct``` is the container for all the included data received
/// after a submit POST.
#[derive(Deserialize, Debug)]
pub struct Included {
    #[serde(rename="attributes")]
    pub included_attrbiutes: IncludedAttributes
}