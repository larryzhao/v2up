use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    LoadSettingsError,
    WorkdirUninitialized,
    CreateFileError,
    ReadFileError,
    WriteFileError,
    EncodeYAMLError,
    ParseJSONError,
    HTTPRequestError,
    Base64DecodeError,
    UnknownServerProtocol,
    GetCurrentProcessIDError,
    ExecuteCommandError,
    TemplateNotFound,
    RenderTemplateNotFound,
    InvalidPath,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
