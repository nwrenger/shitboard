#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Custom(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Custom(err)
    }
}

pub fn error_message(error: &str) -> String {
    match error {
        "Arguments" => "The user provided arguments are malformed",
        "FileOpen" => "A file could not be found or opened",
        "AlreadyExists" => "A file with that name already exists",
        "InvalidFileType" => "An uploaded file has an invalid type",
        "Network" => "Could not connect to server",
        "InvalidFormat" => "Invalid file format",
        "NothingFound" => "No matching results",
        "Conversion" => "Conversion error, decoding, ...",
        _ => "An unknown error has occurred.\nTry refreshing the page!",
    }
    .to_string()
}
