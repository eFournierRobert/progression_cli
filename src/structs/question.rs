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
    pub id: String,
    pub attributes: Option<Attributes>
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Attributes {
    pub auteur: Option<String>,
    pub description: Option<String>,
    pub niveau: Option<String>,
    pub titre: Option<String>,
    pub énoncé: Option<String>,
    pub licence: Option<String>,
    pub code: Option<String>,
    pub langage: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Included {
    pub attributes: Attributes
}