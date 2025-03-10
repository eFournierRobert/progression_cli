#[cfg(test)]
mod tests {
    use crate::question::deserialize::deserialize_question;

    #[test]
    fn test_given_a_simple_prog_question_with_a_single_language_when_we_pass_the_json_of_the_question_to_deserialize_we_get_its_attributes(
    ) {
        let json_string = r#"
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
                    "attributes": {
                        "code": "def salutations(x):\n    for _ in range(x):\n    print('Bonjour le monde!')\n\nsalutions(3)",
                        "langage": "python"
                    }
                }
            ]
        }"#;

        let result = deserialize_question(json_string.to_string());

        assert!(result.is_ok());

        let question = result.unwrap();

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

        assert_eq!(question.included.len(), 1);
        assert_eq!(
            question.included[0].attributes.code,
            Some("def salutations(x):\n    for _ in range(x):\n    print('Bonjour le monde!')\n\nsalutions(3)".to_string())
        );
        assert_eq!(
            question.included[0].attributes.langage,
            Some("python".to_string())
        );
    }

    #[test]
    fn test_given_a_simple_prog_question_with_multiple_languages_when_we_pass_the_json_of_the_question_to_deserialize_we_get_its_attributes(
    ) {
        let json_string = r#"
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
                    "attributes": {
                        "code": "def salutations(x):\n    for _ in range(x):\n        print('Bonjour le monde!')\n\nsalutions(3)",
                        "langage": "python"
                    } 
                },
                {
                    "attributes": {
                        "code": "fn salutations(x: u32) {\n    for _ in 0..x {\n        println!('Bonjour le monde!');\n    }\n}\n\nfn main() {\n    salutations(3);\n}",
                        "langage": "rust"
                    }
                }
            ]
        }"#;

        let result = deserialize_question(json_string.to_string());

        assert!(result.is_ok());

        let question = result.unwrap();

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

        assert_eq!(question.included.len(), 2);
        assert_eq!(
            question.included[0].attributes.code,
            Some("def salutations(x):\n    for _ in range(x):\n        print('Bonjour le monde!')\n\nsalutions(3)".to_string())
        );
        assert_eq!(
            question.included[0].attributes.langage,
            Some("python".to_string())
        );
        assert_eq!(
            question.included[1].attributes.code,
            Some("fn salutations(x: u32) {\n    for _ in 0..x {\n        println!('Bonjour le monde!');\n    }\n}\n\nfn main() {\n    salutations(3);\n}".to_string())
        );
        assert_eq!(
            question.included[1].attributes.langage,
            Some("rust".to_string())
        );
    }
}
