use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Question {
    pub data: Data,
    included: Option<Vec<Included>>
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Data {
    id: Option<String>,
    attributes: Option<Attributes>
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Attributes {
    auteur: Option<String>,
    description: Option<String>,
    niveau: Option<String>,
    titre: Option<String>,
    énoncé: Option<String>,
    code: Option<String>,
    langage: Option<String>
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Included {
    attributes: Option<Attributes>
}