use crate::structs::question::Question;

mod deserialize;

pub fn complete_json_single_language_exercise() -> &'static str {
    r#"
        {
            "data": {
                "id": "1",
                "attributes": {
                    "auteur": "Albert Einstein",
                    "description": "Ceci est une question simple avec un langage",
                    "niveau": "débutant",
                    "titre": "Appeler une fonction paramétrée",
                    "énoncé": "La fonction `salutations` affiche une salution autant de fois que la valeur reçue en paramètre. Utilisez-la pour faire afficher «Bonjour le monde!» autant de fois que le nombre reçu en entrée.",
                    "licence": "poétique"
                }
            },
            "included": [
                {
                    "type": "code",
                    "attributes": {
                        "code": "def salutations(x):\n    for _ in range(x):\n        print('Bonjour le monde!')\n\nsalutations(3)",
                        "langage": "python"
                    }
                }
            ]
        }"#
}

pub fn complete_json_multiple_languages_exercise() -> &'static str {
    r#"
        {
            "data": {
                "id": "1",
                "attributes": {
                    "auteur": "Albert Einstein",
                    "description": "Ceci est une question simple avec un langage",
                    "niveau": "débutant",
                    "titre": "Appeler une fonction paramétrée",
                    "énoncé": "La fonction `salutations` affiche une salution autant de fois que la valeur reçue en paramètre. Utilisez-la pour faire afficher «Bonjour le monde!» autant de fois que le nombre reçu en entrée.",
                    "licence": "poétique"
                }
            },
            "included": [
                {
                    "type": "code",
                    "attributes": {
                        "code": "def salutations(x):\n    for _ in range(x):\n        print('Bonjour le monde!')\n\nsalutations(3)",
                        "langage": "python"
                    }
                },
                {
                    "type": "code",
                    "attributes": {
                        "code": "fn salutations(x: u32) {\n    for _ in 0..x {\n        println!('Bonjour le monde!');\n    }\n}\n\nfn main() {\n    salutations(3);\n}",
                        "langage": "rust"
                    }
                }
            ]
        }"#
}

pub fn minimalist_json_single_language_exercise() -> &'static str {
    r#"
        {
            "data": {
                "id": "1",
                "attributes": {
                    "auteur": null,
                    "description": null,
                    "niveau": null,
                    "titre": null,
                    "énoncé": null,
                    "licence": null
                }
            },
            "included": [
                {
                    "type": "code",
                    "attributes": {
                        "code": "code",
                        "langage": "lang"
                    }
                }
            ]
        }"#
}

pub fn assert_json_fields(question: &Question) {
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

    assert_eq!(
        question.included[0].included_attributes.code,
        Some("def salutations(x):\n    for _ in range(x):\n        print('Bonjour le monde!')\n\nsalutations(3)".to_string())
        );
    assert_eq!(
        question.included[0].included_attributes.langage,
        Some("python".to_string())
    );
}
