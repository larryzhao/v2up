pub enum ErrorKind {
    FileNotFound,
    ReadFileError,
    WriteFileError,
    ParseYAMLError,
    EncodeYAMLError,
    ParseJSONError,
    CommandNotFoundError,
}
