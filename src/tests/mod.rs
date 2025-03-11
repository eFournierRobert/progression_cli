mod api;
mod multi_file_exercise;
mod single_file_exercise;

use crate::structs::question::Question;

pub fn assert_json_fields_question_base(question: &Question) {
    assert_eq!(question.data.id, "1");
    assert_eq!(
        question.data.attributes.as_ref().unwrap().auteur,
        Some("Albert Einstein".to_string())
    );
    assert_eq!(
        question.data.attributes.as_ref().unwrap().description,
        Some("Ceci est une question simple avec un langage".to_string())
    );
    assert_eq!(
        question.data.attributes.as_ref().unwrap().niveau,
        Some("débutant".to_string())
    );
    assert_eq!(
        question.data.attributes.as_ref().unwrap().titre,
        Some("Appeler une fonction paramétrée".to_string())
    );
    assert_eq!(
            question.data.attributes.as_ref().unwrap().énoncé,
            Some("La fonction `salutations` affiche une salution autant de fois que la valeur reçue en paramètre. Utilisez-la pour faire afficher «Bonjour le monde!» autant de fois que le nombre reçu en entrée.".to_string())
        );
    assert_eq!(
        question.data.attributes.as_ref().unwrap().licence,
        Some("poétique".to_string())
    );
}

pub fn assert_null_json_fields(question: &Question) {
    question.data.attributes.iter().for_each(|attribute| {
        assert!(attribute.auteur.is_none());
        assert!(attribute.description.is_none());
        assert!(attribute.niveau.is_none());
        assert!(attribute.titre.is_none());
        assert!(attribute.énoncé.is_none());
        assert!(attribute.licence.is_none());
    });
}
