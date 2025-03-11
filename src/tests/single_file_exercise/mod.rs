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
                    "type": "prog",
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
                    "type": "prog",
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
                    "type": "prog",
                    "attributes": {
                        "code": "code",
                        "langage": "langage"
                    }
                }
            ]
        }"#
}

pub fn nombre_langue(question: &Question) -> usize {
    return question
        .included
        .iter()
        .filter(|attribute| attribute.included_attributes.langage.is_some())
        .count();
}

pub fn nombre_attributes(question: &Question) -> usize {
    return question.included.len();
}
