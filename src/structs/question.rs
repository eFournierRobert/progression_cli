use serde::Deserialize;

/// Question ```struct```.
/// 
/// This ```struct``` is the container for all the data regarding a Question.
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Question {
    pub data: Data,
    pub included: Vec<Included>
}

/// Data ```struct```.
/// 
/// This ```struct``` contains some data regarding a Question.
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Data {
    pub id: String,
    pub attributes: Option<Attributes>
}

/// Attributes ```struct```.
/// 
/// This ```struct``` is the container for all the useful attributes for a Question.
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Attributes {
    pub auteur: Option<String>,
    pub description: Option<String>,
    pub niveau: Option<String>,
    pub titre: Option<String>,
    pub énoncé: Option<String>,
    pub licence: Option<String>
}

/// IncludedAttributes ```struct```.
/// 
/// This ```struct``` is the Attributes in the included data for a Question.
#[derive(Deserialize, Debug, Clone)]
pub struct IncludedAttributes {
    pub code: String,
    pub langage: String
}

/// Included ```struct```.
/// 
/// This ```struct``` contains the included data when getting a Question.
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Included {
    #[serde(rename="attributes")]
    pub included_attributes: IncludedAttributes
}