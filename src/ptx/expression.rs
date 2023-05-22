use super::{PtxReader, error::*};

pub enum OuterToken {
    Version,
    Target,
    AddressSize,
    Func,
    Global,
    Visible,
}

#[derive(Debug)]
pub enum Comment {
    Line(usize, String),
    Lines { start: usize, end: usize, comment: Vec<String> },
}

impl PtxReader {
    pub fn push_line_comment(&mut self) -> Result<Option<OuterToken>, PtxError> {
        self.comments.push(Comment::Line(
            self.num,
            String::from_utf8(
                std::mem::replace(
                    &mut self.line, 
                    Vec::new()
        ))?));
        Ok(None)
    }
}
    

#[derive(Debug)]
pub struct DeclaredFunction {
    retvalue: String,
    name: String,
    params: Vec<String>,
}

