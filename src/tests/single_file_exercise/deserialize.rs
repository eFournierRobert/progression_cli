#[cfg(test)]
mod tests {
    use crate::question::deserialize::deserialize_question;
    use crate::tests::single_file_exercise::{
        assert_json_fields, complete_json_multiple_languages_exercise,
        complete_json_single_language_exercise, minimalist_json_single_language_exercise,
    };

    #[test]
    fn test_given_a_simple_prog_question_with_a_single_language_when_we_pass_the_json_of_the_question_to_deserialize_we_get_its_attributes(
    ) {
        let json_string = complete_json_single_language_exercise();

        let result = deserialize_question(json_string.to_string());

        assert!(result.is_ok());

        let question = result.unwrap();

        assert_json_fields(&question);
    }

    #[test]
    fn test_given_a_simple_prog_question_with_multiple_languages_when_we_pass_the_json_of_the_question_to_deserialize_we_get_its_attributes(
    ) {
        let json_string = complete_json_multiple_languages_exercise();

        let result = deserialize_question(json_string.to_string());

        assert!(result.is_ok());

        let question = result.unwrap();

        assert_json_fields(&question);

        assert_eq!(
            question.included[1].attributes.code,
            Some("fn salutations(x: u32) {\n    for _ in 0..x {\n        println!('Bonjour le monde!');\n    }\n}\n\nfn main() {\n    salutations(3);\n}".to_string())
        );
        assert_eq!(
            question.included[1].attributes.langage,
            Some("rust".to_string())
        );
    }

    #[test]
    fn test_given_a_simple_minimalist_prog_question_with_a_single_language_when_we_pass_the_json_of_the_question_to_deserialize_we_get_its_attributes(
    ) {
        let json_string = minimalist_json_single_language_exercise();

        let result = deserialize_question(json_string.to_string());

        assert!(result.is_ok());

        let question = result.unwrap();

        question.data.attributes.iter().for_each(|attribute| {
            assert!(attribute.auteur.is_none());
            assert!(attribute.description.is_none());
            assert!(attribute.niveau.is_none());
            assert!(attribute.titre.is_none());
            assert!(attribute.énoncé.is_none());
            assert!(attribute.licence.is_none());
        });

        question.included.iter().for_each(|included| {
            assert!(!included.attributes.code.is_none());
            assert!(!included.attributes.code.is_none());
        });
    }
}
