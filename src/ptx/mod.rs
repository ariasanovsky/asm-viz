use std::{path::PathBuf, fs::File, io::{BufReader}};

pub mod error;
pub mod expression;
pub mod read;
use error::*;
use expression::*;

#[derive(Debug, Default)]
pub struct PtxReader {
    reader: Option<BufReader<File>>,
    buffer: Vec<u8>,
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
        self.preamble()?;
        self.body()
    }

    fn preamble(&mut self) -> Result<(), PtxError> {
        let version = self.version()?;
        //println!("version = {version}");
        let target = self.target()?;
        //println!("target = {target}");
        let address_size = self.address_size()?;
        //println!("address_size = {address_size}");
        self.metadata = Some(Metadata { version, target, address_size });
        println!("{self:?}");
        Ok(())
    }

    fn body(&mut self) -> Result<(), PtxError> {
        match self.outer_token()? {
            token @ (
                OuterToken::Version |
                OuterToken::Target  |
                OuterToken::AddressSize
            ) => Err(PtxError::MetadataTokenAfterPreamble(token)),
            OuterToken::Function => self.function(),
            OuterToken::Global => todo!(),
            OuterToken::Visible => todo!(),
        }
    }
    
    fn function(&mut self) -> Result<(), PtxError> {
        let signature = self.signature()?;
        todo!()
    }

    fn signature(&mut self) -> Result<Signature, PtxError> {
        let mut signature = self.trimmed_drain_buffer()?;
        println!("sig = {signature}");
        let mut ret_value: Option<String> = None;
        match (signature.starts_with('('), signature.find(')')) {
            (true, None) => todo!("look for ')' on next line"),
            (true, Some(pos)) => {
                let splits = signature.split_at(pos);
                ret_value = Some(splits.0[1..].into());
                signature = splits.1[1..].trim_start().into();
            },
            (false, _) => todo!(),
        }
        println!("ret_value = {ret_value:?}");
        println!("signature = {signature:?}");
        todo!()
    }

    fn version(&mut self) -> Result<String, PtxError> {
        match self.outer_token()? {
            OuterToken::Version => Ok(self.drain_buffer()?),
            token => Err(PtxError::OuterTokenOrder(
                token, OuterToken::Version
            ))
        }
    }

    fn target(&mut self) -> Result<String, PtxError> {
        match self.outer_token()? {
            OuterToken::Target => Ok(self.drain_buffer()?),
            token => Err(PtxError::OuterTokenOrder(
                token, OuterToken::Version
            ))
        }
    }

    fn address_size(&mut self) -> Result<String, PtxError> {
        match self.outer_token()? {
            OuterToken::AddressSize => Ok(self.drain_buffer()?),
            token => Err(PtxError::OuterTokenOrder(
                token, OuterToken::Version
            ))
        }
    }

    fn outer_expression(&mut self) -> Result<Option<OuterToken>, PtxError> {
        let delim = self.open_delimeter()?;
        println!("delim = {delim:?}");
        self._show_line();
        match delim {
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
    
    fn _show_line(&self) {
        println!("line = {:?}", String::from_utf8(self.buffer.clone()));
    }

    pub fn outer_token(&mut self) -> Result<OuterToken, PtxError> {
        if let Some(token) = self.outer_expression()? {
            Ok(token)
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