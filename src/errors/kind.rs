pub enum ErrorKind {
    FileNotFound,
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
}
