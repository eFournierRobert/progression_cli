#[cfg(test)]
mod tests {
    use reqwest::{blocking::get, StatusCode};
    use serde_json::Value;

    use crate::utils::get_api_url;

    #[test]
    fn test_given_the_url_api_of_progression_when_a_get_request_is_made_we_get_a_response_from_the_api(
    ) {
        let url_api = get_api_url();

        let response = get(url_api);

        match response {
            Ok(res) => {
                assert!(res.status().is_success());
            }
            Err(e) => {
                print!("Nothing: {}", e)
            }
        }
    }

    #[test]
    fn test_given_the_api_endpoint_for_fetching_a_question_when_a_get_request_is_made_we_get_the_reponse_200_and_question_data(
    ) {
        let api_url = get_api_url();
        let question_uri = "aHR0cHM6Ly9wcm9ncmVzc2lvbi5wYWdlcy5kdGkuY3Jvc2Vtb250LnF1ZWJlYy9jb250ZW51L2RlbW8vZGExNDNjMTEtZjc2My00MDY0LWJjMDMtNTYxZGNkODFjNmUyL2luZm8ueW1s";
        let full_url = format!(
            "{}question/{}?include=questions,ebauches",
            api_url, question_uri
        );

        let response = get(&full_url);

        match response {
            Ok(res) => {
                assert_eq!(
                    res.status(),
                    StatusCode::OK,
                    "API did not respond with 200 OK"
                );

                let json: Result<Value, _> = res.json();
                assert!(json.is_ok(), "Failed to parse response as JSON");

                let json = json.unwrap();
                assert!(
                    json.get("data").is_some(),
                    "Response does not contain 'data' field"
                );
                assert!(
                    json.get("included").is_some(),
                    "Response does not contain 'included' field"
                );
            }
            Err(e) => {
                panic!("Nothing: {}", e);
            }
        }
    }
}
