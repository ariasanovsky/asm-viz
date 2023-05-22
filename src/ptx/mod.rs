use std::{path::PathBuf, fs::File, io::{BufReader}};

pub mod error;
pub mod expression;
pub mod read;
use error::*;
use expression::*;

#[derive(Debug, Default)]
pub struct PtxReader {
    reader: Option<BufReader<File>>,
    line: Vec<u8>,
    num: usize,
    comments: Vec<Comment>,
    metadata: Option<Metadata>,
    //state: ReaderState,
}

impl TryFrom<PathBuf> for PtxReader {
    type Error = PtxError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self {
            reader: Some(BufReader::new(File::open(path)?)),
            ..Default::default()
        })
    }
}

// #[derive(Debug)]
// enum ReaderState {
//     New,
//     Version { version: String },
//     VersionAndTarget { version: String, target: String },
//     Metadata { data: Metadata,  },
// }

#[derive(Debug)]
struct Metadata {
    version: String,
    target: String,
    address_size: String,
}

// impl Default for ReaderState {
//     fn default() -> Self {
//         Self::New
//     }
// }

#[derive(Debug)]
pub enum OpenDelimeter {
    EndOfFile,
    Period,
    LineComment,
    CommentOpen,
}

impl PtxReader {
    pub fn populate(&mut self) -> Result<(), PtxError> {
        self.populate_metadata()?;
        self.populate_to_end()
    }

    fn populate_metadata(&mut self) -> Result<(), PtxError> {
        let version = self.version()?;
        let target = self.target()?;
        let address_size = self.address_size()?;
        self.metadata = Some(Metadata { version, target, address_size });
        Ok(())
    }

    fn populate_to_end(&mut self) -> Result<(), PtxError> {
        todo!()
    }
    
    fn version(&mut self) -> Result<String, PtxError> {
        match self.outer_token()? {
            OuterToken::Version => Ok(
                "foo".into()
            ),
            token => Err(PtxError::OuterTokenOrder(
                token, OuterToken::Version
            ))
        }
    }

    fn target(&mut self) -> Result<String, PtxError> {
        todo!()
    }

    fn address_size(&mut self) -> Result<String, PtxError> {
        todo!()
    }

    fn outer_expression(&mut self) -> Result<Option<OuterToken>, PtxError> {
        let delim = self.open_delimeter()?;
        println!("delim = {delim:?}");
        println!("buffer = {:?}", self.line);
        match self.open_delimeter()? {
            OpenDelimeter::LineComment => self.push_line_comment(),
            OpenDelimeter::Period => {
                self
                    .token_string()
                    .as_str()
                    .try_into()
                    .map(Some)
            }
            token => todo!("token {token:?}"),
        }
    }

    pub fn outer_token(&mut self) -> Result<OuterToken, PtxError> {
        if let Some(token) = self.outer_expression()? {
            todo!()
        } else {
            self.outer_token()
        }

    }

    fn open_delimeter(&mut self) -> Result<OpenDelimeter, PtxError> {
        match self.char()? {
            '.' => Ok(OpenDelimeter::Period),
            '/' => {
                match self.char_on_line()
                    .ok_or(PtxError::UnmatchedForwardSlash)?
                {
                    '/' => Ok(OpenDelimeter::LineComment),
                    '*' => Ok(OpenDelimeter::CommentOpen),
                    _ => Err(PtxError::NotForwardSlashOrStar),
                }
            },
            c if c.is_whitespace() => self.open_delimeter(),
            c => todo!("char {c}"),
        }
    }
}