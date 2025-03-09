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
/// after a submit POST. **Note that feedback, réussi and tests_réussis
/// are all under SubmitResponse.data.attributes and résultat, 
/// sortie_erreur, sortie_observée and temps_exécution are all under
/// SubmitResponse.included.attributes**.
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

/// Included ```struct```.
/// 
/// This ```struct``` is the container for all the data received
/// after a submit POST.
/// 
/// Only used to get the résultat, sortie_erreur, sortie_observée and 
/// temps_exécution in the JSON returned from the API.
/// **All the other attributes are store inside ```Data```**.
#[derive(Deserialize, Debug)]
pub struct Included {
    pub attributes: Attributes
}