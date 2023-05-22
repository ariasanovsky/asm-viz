use super::expression::OuterToken;

#[derive(Debug)]
pub enum PtxError {
    IoError(std::io::Error),
    FromUtf8Error(std::string::FromUtf8Error),
    
    InvalidOuterToken,
    OuterTokenOrder(OuterToken, OuterToken),
    MetadataTokenAfterPreamble(OuterToken),

    NoLoadedBuffer,
    NotForwardSlashOrStar,
    UnmatchedForwardSlash,
}

impl From<std::io::Error> for PtxError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<std::string::FromUtf8Error> for PtxError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8Error(value)
    }
}
