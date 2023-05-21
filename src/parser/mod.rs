use std::{path::PathBuf, fs::File, io::{Read, BufReader, BufRead}, convert};

#[derive(Debug)]
enum ReaderState {
    New,
    NeedsVersion,
    NeedsTarget { version: String },
    NeedsAddressSize { version: String, target: String },
    NeedsOuterLexeme { data: Metadata,  },
}

#[derive(Debug)]
struct Metadata {
    version: String,
    target: String,
    address_size: String,
}

impl Default for ReaderState {
    fn default() -> Self {
        Self::New
    }
}

#[derive(Debug)]
pub struct PtxReader {
    state: ReaderState,
    reader: BufReader<File>,
    buffer: Vec<u8>,
}

struct _DeclaredFunction {
    retvalue: String,
    name: String,
    params: Vec<String>,
}

impl TryFrom<PathBuf> for PtxReader {
    type Error = std::io::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        Ok(Self {
            state: Default::default(),
            reader: BufReader::new(File::open(path)?),
            buffer: vec![]
        })
    }
}

#[derive(Debug)]
pub enum ReadError {
    IoError(std::io::Error),
    NotForwardSlashOrStar,
    UnmatchedForwardSlash,
    InvalidOuterLexeme,
}

impl From<std::io::Error> for ReadError {
    fn from(value: std::io::Error) -> Self {
        ReadError::IoError(value)
    }
}

pub enum OuterLexeme {
    Version,
    Target,
    AddressSize,
    Func,
    Global,
    Visible,
}

#[derive(Debug)]
pub enum OpenDelimeter {
    EndOfFile,
    Period,
    LineComment,
    CommentOpen,
}

impl PtxReader {
    pub fn outer_expression(&mut self) -> Result<OuterLexeme, ReadError> {
        let delim = self.open_delimeter()?;
        println!("delim = {delim:?}");
        println!("buffer = {:?}", self.buffer);
        match self.open_delimeter()? {
            token @ OpenDelimeter::EndOfFile => todo!("what to do with token {token:?}"),
            token @ OpenDelimeter::Period => todo!("what to do with token {token:?}"),
            token @ OpenDelimeter::LineComment => todo!("what to do with token {token:?}"),
            token @ OpenDelimeter::CommentOpen => todo!("what to do with token {token:?}"),
        }
    }

    pub fn open_delimeter(&mut self) -> Result<OpenDelimeter, ReadError> {
        match self.char()? {
            '.' => Ok(OpenDelimeter::Period),
            '/' => {
                match self.char()? {
                    '/' => Ok(OpenDelimeter::LineComment),
                    '*' => Ok(OpenDelimeter::CommentOpen),
                    _ => Err(ReadError::NotForwardSlashOrStar),
                }
            },
            c if c.is_whitespace() => self.open_delimeter(),
            c => todo!("what to do with {c}"),
        }
    }

    pub fn char(&mut self) -> Result<char, ReadError> {
        while self.buffer.is_empty() {
            while 1 == self.reader
                .read_until(b'\n', &mut self.buffer)
                .map_err(Into::<ReadError>::into)?
            {
                println!("read nothing!")
            }

        }
        
        Ok(self.buffer.remove(0).into())
    }
}