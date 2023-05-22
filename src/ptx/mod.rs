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
    Metadata: Option<Metadata>,
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
        let data = self.populate_metadata()?;
        println!("{data:?}");
        self.populate_to_end()
    }

    fn populate_metadata(&mut self) -> Result<(), PtxError> {
        let version = self.version()?;
        let target = self.target()?;
        let address_size = self.address_size()?;
        todo!()
    }

    fn populate_to_end(&mut self) -> Result<(), PtxError> {
        todo!()
    }
    
    fn version(&mut self) -> Result<String, PtxError> {
        todo!()
    }

    fn target(&mut self) -> Result<String, PtxError> {
        todo!()
    }

    fn address_size(&mut self) -> Result<String, PtxError> {
        todo!()
    }

    pub fn outer_expression(&mut self) -> Result<Option<OuterToken>, PtxError> {
        let delim = self.open_delimeter()?;
        println!("delim = {delim:?}");
        println!("buffer = {:?}", self.line);
        match self.open_delimeter()? {
            OpenDelimeter::LineComment => self.push_line_comment(),
            OpenDelimeter::Period => {
                todo!()
            }
            token => todo!("token {token:?}"),
        }
    }

    pub fn outer_token(&mut self) -> Result<OuterToken, PtxError> {
        todo!()
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