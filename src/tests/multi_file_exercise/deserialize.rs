#[cfg(test)]
mod tests {
    use crate::question::deserialize::deserialize_question;
    use crate::tests::multi_file_exercise::{
        complete_json_multiple_files_multiple_languages_exercise,
        complete_json_multiple_files_single_language_exercise,
        minimalist_json_multiple_files_single_language_exercise,
    };
    use crate::tests::{assert_json_fields_question_base, assert_null_json_fields};

    #[test]
    fn test_given_a_multi_file_prog_question_with_a_single_language_when_we_pass_the_json_of_the_question_to_deserialize_we_get_its_attributes(
    ) {
        let json_string = complete_json_multiple_files_single_language_exercise();

        let result = deserialize_question(json_string.to_string());

        assert!(result.is_ok());

        let question = result.unwrap();

        assert_json_fields_question_base(&question);
    }

    #[test]
    fn test_given_a_multi_file_prog_question_with_multiple_languages_when_we_pass_the_json_of_the_question_to_deserialize_we_get_its_attributes(
    ) {
        let json_string = complete_json_multiple_files_multiple_languages_exercise();

        let result = deserialize_question(json_string.to_string());

        assert!(result.is_ok());

        let question = result.unwrap();

        assert_json_fields_question_base(&question);
    }

    #[test]
    fn test_given_a_multi_file_minimalist_prog_question_with_a_single_language_when_we_pass_the_json_of_the_question_to_deserialize_we_get_its_attributes(
    ) {
        let json_string = minimalist_json_multiple_files_single_language_exercise();

        let result = deserialize_question(json_string.to_string());

        assert!(result.is_ok());

        let question = result.unwrap();
        assert_null_json_fields(&question);
    }
}
