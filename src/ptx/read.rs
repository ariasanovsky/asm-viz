use std::io::BufRead;

use super::{PtxReader, error::*};

impl PtxReader {
    pub fn char(&mut self) -> Result<char, PtxError> {
        while self.line.is_empty() {
            while 1 == self.load_line()?
            {
                println!("read nothing!")
            }

        }
        
        Ok(self.line.remove(0).into())
    }

    pub fn load_line(&mut self) -> Result<usize, PtxError> {
        self.num += 1;
        self.reader
            .as_mut()
            .ok_or(PtxError::NoLoadedBuffer)?
            .read_until(b'\n', &mut self.line)
            .map_err(Into::into)
    }

    pub fn char_on_line(&mut self) -> Option<char> {
        if self.line.is_empty() {
            None
        } else {
            Some(self.line.remove(0).into())
        }
    }

    pub fn token_string(&mut self) -> String {
        let mut token = String::new();
        println!("token_string w/ line = {:?}", self.line);
        while let Some(c) = self.char_on_line() {
            if c == ' ' {
                break
            } else {
                token.push(c)
            }
        }
        token
    }
}
