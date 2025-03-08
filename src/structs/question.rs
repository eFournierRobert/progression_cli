use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Question {
    pub data: Data,
    pub included: Vec<Included>
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Data {
    pub id: Option<String>,
    pub attributes: Option<Attributes>
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Attributes {
    pub auteur: Option<String>,
    pub description: Option<String>,
    pub niveau: Option<String>,
    pub titre: Option<String>,
    énoncé: Option<String>,
    pub code: Option<String>,
    pub langage: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Included {
    pub attributes: Attributes
}