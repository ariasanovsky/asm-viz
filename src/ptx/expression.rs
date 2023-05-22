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
pub struct DeclaredFunction {
    ret_value: Option<String>,
    name: String,
    parameters: Vec<String>,
}

#[derive(Debug)]
pub struct Signature {

}