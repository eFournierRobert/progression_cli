use serde::Deserialize;

/// SubmitTestResponse ```struct```.
/// 
/// This ```struct``` is the container for the data
/// received in response of a test POST.
#[derive(Deserialize)]
pub struct SubmitTestResponse {
    pub data: Data,
}

/// Data ```struct```.
/// 
/// This ```struct``` contains all the data received after a test POST.
#[derive(Deserialize)]
pub struct Data {
    pub attributes: Attributes,
}

/// Attributes ```struct```.
/// 
/// This ```struct``` contains all the necessary attributes
/// received from the API after a test POST.
#[derive(Deserialize)]
pub struct Attributes {
    pub sortie_observée: Option<String>,
    pub sortie_erreur: Option<String>,
    pub résultat: bool,
    pub feedback: Option<String>,
    pub temps_exécution: u16,
}
