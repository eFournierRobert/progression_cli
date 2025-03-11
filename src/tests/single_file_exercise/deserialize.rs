#[cfg(test)]
mod tests {
    use crate::question::deserialize::deserialize_question;
    use crate::tests::single_file_exercise::{
        complete_json_multiple_languages_exercise, complete_json_single_language_exercise,
        minimalist_json_single_language_exercise, nombre_attributes, nombre_langue,
    };
    use crate::tests::{assert_json_fields_question_base, assert_null_json_fields};

    #[test]
    fn test_given_a_simple_prog_question_with_a_single_language_when_we_pass_the_json_of_the_question_to_deserialize_we_get_its_attributes(
    ) {
        let json_string = complete_json_single_language_exercise();

        let result = deserialize_question(json_string.to_string());

        assert!(result.is_ok());

        let question = result.unwrap();

        assert_json_fields_question_base(&question);
        assert_eq!(nombre_langue(&question), 1);
        assert_eq!(nombre_attributes(&question), 1);
    }

    #[test]
    fn test_given_a_simple_prog_question_with_multiple_languages_when_we_pass_the_json_of_the_question_to_deserialize_we_get_its_attributes(
    ) {
        let json_string = complete_json_multiple_languages_exercise();

        let result = deserialize_question(json_string.to_string());

        assert!(result.is_ok());

        let question = result.unwrap();

        assert_json_fields_question_base(&question);
        assert_eq!(nombre_langue(&question), 2);
        assert_eq!(nombre_attributes(&question), 2);
    }

    #[test]
    fn test_given_a_simple_minimalist_prog_question_with_a_single_language_when_we_pass_the_json_of_the_question_to_deserialize_we_get_its_attributes(
    ) {
        let json_string = minimalist_json_single_language_exercise();

        let result = deserialize_question(json_string.to_string());

        assert!(result.is_ok());

        let question = result.unwrap();
        assert_null_json_fields(&question);
    }

    #[test]
    fn test_given_an_invalid_json_when_we_pass_the_json_to_deserialize_we_get_an_error() {
        let json_string = "json";

        let result = deserialize_question(json_string.to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_given_an_invalid_json_with_missing_fields_when_we_pass_the_json_to_deserialize_we_get_an_error(
    ) {
        let json_string = r#"
        {
            "data": {
                "id": "1"
            }
        }
        "#;

        let result = deserialize_question(json_string.to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_given_an_empty_json_with_when_we_pass_the_json_to_deserialize_we_get_an_error() {
        let json_string = "{}";

        let result = deserialize_question(json_string.to_string());

        assert!(result.is_err());
    }
}
