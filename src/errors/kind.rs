use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    FileNotFound,
    LoadSettingsError,
    WorkdirUnavailable,
    WorkdirUninitialized,
    DirNotFound,
    DirEmpty,
    DirAlreadyTaken,
    CreateFileError,
    ReadFileError,
    WriteFileError,
    ParseYAMLError,
    EncodeYAMLError,
    ParseJSONError,
    CommandNotFoundError,
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
