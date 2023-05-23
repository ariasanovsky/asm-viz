use super::{PtxReader, error::*};

#[derive(Debug)]
pub enum OuterToken {
    Version,
    Target,
    AddressSize,
    Function,
    Global,
    Visible,
}

impl TryFrom<&str> for OuterToken {
    type Error = PtxError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "version" => OuterToken::Version,
            "target" => OuterToken::Target,
            "address_size" => OuterToken::AddressSize,
            "func" => OuterToken::Function,
            "global" => OuterToken::Global,
            "visible" => OuterToken::Visible,
            _ => return Err(PtxError::InvalidOuterToken),
        })
    }
}

#[derive(Debug)]
pub enum Comment {
    Line(usize, String),
    Lines { start: usize, end: usize, comment: Vec<String> },
}

impl PtxReader {
    pub fn push_line_comment(&mut self) -> Result<Option<OuterToken>, PtxError> {
        let comment = self.drain_buffer()?;
        self.comments.push(Comment::Line(
            self.num,
            comment
        ));

        //println!("new comment! all comments:\n{:?}", self.comments);

        Ok(None)
    }
}
    

#[derive(Debug)]
pub struct Signature {
    pub(super) return_value: Option<String>,
    pub(super) name: String,
    pub(super) parameters: Vec<Parameter>,
}

#[derive(Debug)]
pub struct Parameter {
    data_type: String,
    name: String,
}

#[derive(Debug)]
pub struct RawParameters {
    pub(super) value: String,
}

impl From<String> for RawParameters {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl TryFrom<RawParameters> for Vec<Parameter> {
    type Error = PtxError;

    fn try_from(RawParameters {value}: RawParameters) -> Result<Self, Self::Error> {
        let mut parameters = vec![];
        let mut tokens = value.split_whitespace();
        while let Some(keyword) = tokens.next() {
            if keyword != ".param" {
                return Err(PtxError::MissingParamKeyword)
            }
            let data_type = tokens
                .next()
                .ok_or(PtxError::MissingDatatype)?
                .into();
            let name = tokens
                .next()
                .ok_or(PtxError::MissingParamName)?
                .into();
            parameters.push(Parameter { data_type, name })
        }
        Ok(parameters)
    }
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    return_value: Option<String>,
    name: String,
    parameters: Vec<Parameter>,
}

impl From<Signature> for FunctionDeclaration {
    fn from(value: Signature) -> Self {
        let Signature { return_value, name, parameters } = value;
        Self { return_value, name, parameters }
    }
}

pub struct GlobalVariable {
    alignment: usize,
    data_type: String,
    name: String,
    size: Option<usize>,
    initialization: Option<String>,
}

// todo!("convert from destructive token streams instead of String...")
impl TryFrom<String> for GlobalVariable {
    type Error = PtxError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}