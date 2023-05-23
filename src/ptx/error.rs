use std::num::ParseIntError;

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

    UnclosedParenthesis,
    MissingOpenParenthesis,
    MissingParamKeyword,
    MissingDatatype,
    MissingParamName,

    MissingAlignKeyword,
    MissingAlignment,
    ParseAlignment(ParseIntError),
    MissingDataType,
    MissingName,
}

impl From<ParseIntError> for PtxError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseAlignment(value)
    }
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
