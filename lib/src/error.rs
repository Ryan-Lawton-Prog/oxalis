use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("could not find {0}")]
    FileNotFound(PathBuf),
    #[error("denied permission to access {0}")]
    PermissionDenied(PathBuf),
    #[error("{0} is not a directory")]
    NotDirectory(PathBuf),
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("error encrypting {0}")]
    Encryption(PathBuf),
    #[error("the type field on the attribute does not match the expected value")]
    IncorrectAttrType,
    #[error("the length file on the attribute does not equal the actual length")]
    IncorrectAttrLen,
    #[error("the size of STUN attribute {0} is too large")]
    AttrTooLarge(&'static str),
    #[error("utf8 decoding error")]
    Utf8Decoding(#[from] std::string::FromUtf8Error),
    #[error("error decoding STUN packet")]
    Decoding,
}