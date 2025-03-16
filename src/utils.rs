use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
};

use colored::Colorize;

#[derive(Debug)]
pub enum RequestError {
    AuthCreationFail,
    QuestionRequestFail,
    QuestionDeserializeFail,
    SubmitSerializeFail,
    SubmitRequestFail,
    SubmitDeserializeFail,
    FailToGetLangage,
}

pub enum FileCreationError {
    FailedCreateDot,
    FailedCreateEnonce,
    FailedCreateQuestion,
    FailedCreateFolder,
    FailedCreateAnswer,
    FailedToWriteTest,
    FolderAlreadyExist,
    FailedToCreateTest,
}

pub enum SubmitError {
    NotInDirectory,
    QuestionFileNotFound,
    CoultNotReadDirectory,
}

/// Reads the URI from the ```.progcli``` file.
///
/// This function reads the current question URI and returns it.
/// Checks at the same time if you're in the directory with the question.
///
/// In case of error, it returns a ```SubmitError::NotInDirectory```.
pub fn read_uri_from_dotfile() -> Result<String, SubmitError> {
    let exist = fs::exists(".progcli").unwrap();

    if exist {
        Ok(fs::read_to_string(".progcli").expect(".progcli not found"))
    } else {
        Err(SubmitError::NotInDirectory)
    }
}

/// Reads the code of the given question file.
///
/// This function reads the code of the given question file and returns it.
pub fn read_code_from_file(file: &String) -> String {
    fs::read_to_string(file).expect("Couldn't read code from {file}")
}

/// A simple getter for the API URL as a String.
///
/// Will return a ```String``` ""https://progression.crosemont.qc.ca/api/v1//"".
pub fn get_api_url() -> String {
    String::from("https://progression.crosemont.qc.ca/api/v1//")
}

/// Gets the filename and type of the question file.
///
/// This function gets the name of the file and its file extension,
/// then puts it inside a HashMap. Keys are ```filename``` and ```filetype```.
///
/// In case of errors, it will return an error from the ```SubmitError``` enum.
pub fn get_question_file_name() -> Result<HashMap<String, String>, SubmitError> {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        match path {
            Ok(res) => {
                let file_name = res.file_name();

                match file_name.to_string_lossy().get(0..8) {
                    Some(x) => {
                        if x == "question".to_string() {
                            let mut ret = HashMap::new();
                            let filename = file_name.to_string_lossy().to_string();

                            ret.insert("filename".to_string(), filename.clone());
                            ret.insert(
                                "filetype".to_string(),
                                filename.get(8..filename.len()).unwrap().to_string(),
                            );

                            return Ok(ret);
                        }
                    }
                    None => {}
                }
            }
            Err(_) => return Err(SubmitError::CoultNotReadDirectory),
        }
    }

    Err(SubmitError::QuestionFileNotFound)
}

/// Ask the user for his username and password.
///
/// This function will prompt the user for his username and password.
/// Then stores it inside an ```Hashmap``` and the ```Hashmap``` inside an ```Result```.
pub fn get_username_password() -> Result<HashMap<String, String>, io::Error> {
    let mut username = String::new();

    print!("Username: ");
    std::io::stdout().flush().unwrap(); // Flushing buffer to print the print! statement.
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read username");
    username.pop(); // Remove newline character

    print!("Password: ");
    std::io::stdout().flush().unwrap(); // Flushing buffer to print the print! statement.
    let password = match rpassword::read_password() {
        Ok(pass) => pass,
        Err(e) => return Err(e),
    };

    let mut hashmap = HashMap::new();
    hashmap.insert(String::from("username"), username.clone());
    hashmap.insert(String::from("password"), password);

    Ok(hashmap)
}

/// Print an error message for the given error.
///
/// This function prints an error message for the given error inside the
/// ```enum``` ```RequestError```.
pub fn request_error_messages(e: RequestError) {
    match e {
        RequestError::AuthCreationFail => {
            println!(
                "{}",
                "Failed to create basic auth.".red().bold()
            );
            return;
        }
        RequestError::QuestionDeserializeFail => {
            println!(
                "{}",
                "Failed to deserialize API response.".red().bold()
            );
            return;
        }
        RequestError::QuestionRequestFail => {
            println!(
                "{}",
                "Failed to make HTTP request for question.".red().bold()
            );
            return;
        }
        RequestError::SubmitRequestFail => {
            println!(
                "{}",
                "Failed to make HTTP request to submit answer or test.".red().bold()
            );
            return;
        }
        RequestError::SubmitSerializeFail => {
            println!(
                "{}",
                "Failed to serialize request body to submit answer or test.".red().bold()
            );
            return;
        }
        RequestError::SubmitDeserializeFail => {
            println!(
                "{}",
                "Failed to deserialize response from submit answer or test.".red().bold()
            );
            return;
        }
        RequestError::FailToGetLangage => {
            println!(
                "{}",
                "Couldn't get language from file type".red().bold()
            );
            return;
        }
    }
}

/// Print an error message for the given error.
///
/// This function prints an error message for the given error inside the
/// ```enum``` ```FileCreationError```.
pub fn file_creation_error_messages(e: FileCreationError) {
    match e {
        FileCreationError::FailedCreateDot => {
            println!(
                "{}",
                "Failed to create .progcli.".red().bold()
            );
            return;
        }
        FileCreationError::FailedCreateEnonce => {
            println!(
                "{}",
                "Failed to create enonce.md file.".red().bold()
            );
            return;
        }
        FileCreationError::FailedCreateQuestion => {
            println!(
                "{}",
                "Failed to create question file.".red().bold()
            );
            return;
        }
        FileCreationError::FailedCreateFolder => {
            println!(
                "{}",
                "Failed to create folder for files.".red().bold()
            );
            return;
        }
        FileCreationError::FailedCreateAnswer => {
            println!(
                "{}",
                "Failed to create answer.md file.".red().bold()
            );
            return;
        }
        FileCreationError::FailedToWriteTest => {
            println!(
                "{}",
                "Failed to write test in enonce.md".red().bold()
            );
            return;
        }
        FileCreationError::FolderAlreadyExist => {
            println!(
                "{}", 
                "Failed to create folder. Folder already exists and is not an empty directory."
                    .red()
                    .bold()
            );
            return;
        }
        FileCreationError::FailedToCreateTest => {
            println!(
                "{}",
                "Failed to create test.md".red().bold()
            );
            return;
        }
    }
}

/// Prints an error message for a ```SubmitError```.
///
/// This function prints an error message depending on
/// the type of the ```SubmitError```.
pub fn print_submit_error_message(e: SubmitError) {
    match e {
        SubmitError::QuestionFileNotFound => {
            println!(
                "{}",
                "Could not find Question file".red().bold()
            );
            return;
        }
        SubmitError::CoultNotReadDirectory => {
            println!(
                "{}",
                "Could not read current directory".red().bold()
            );
            return;
        }
        SubmitError::NotInDirectory => {
            println!(
                "{}",
                "Could not read .progcli. Make sure you are in the directory where you cloned the question."
                    .red().bold()
            );
            return;
        }
    }
}

/// Returns the language for the given file extension.
///
/// This function takes a file extension and returns the
/// programming language for that file extension in an ```Option```.
pub fn get_langage(file_type: &String) -> Option<String> {
    return match file_type.as_str() {
        ".py" => Some(String::from("python")),
        ".java" => Some(String::from("java")),
        ".cs" => Some(String::from("c#")),
        ".rs" => Some(String::from("rust")),
        ".js" => Some(String::from("javascript")),
        ".kt" => Some(String::from("kotlin")),
        _ => None,
    };
}
