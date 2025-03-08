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
/// **Note that ```code``` and ```langage``` are usually stored inside ```Question.included.attributes```
/// and not ```Question.data.attributes```**.
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

/// Included ```struct```.
/// 
/// This ```struct``` is the container for all the data regarding a Question.
/// 
/// Only used to get the code and langage in the JSON returned from the API.
/// **All the other attributes are store inside ```Data```**.
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Included {
    pub attributes: Attributes
}