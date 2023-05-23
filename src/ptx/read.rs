use std::io::BufRead;

use super::{PtxReader, error::*};

impl PtxReader {
    pub fn char(&mut self) -> Result<char, PtxError> {
        while self.buffer.is_empty() {
            while 1 == self.load_line()? {}
        }
        
        Ok(self.buffer.remove(0).into())
    }

    pub fn char_after_whitespace(&mut self) -> Result<char, PtxError> {
        match self.char()? {
            c if c.is_whitespace() => self.char_after_whitespace(),
            c => Ok(c)
        }
    }

    pub fn expression_until(&mut self, end: char) -> Result<String, PtxError> {
        let mut expression = String::new();
        loop {
            match self.char()? {
                c if c != end => expression.push(c),
                _ => return Ok(expression)
            }
        }
    }

    pub fn load_line(&mut self) -> Result<usize, PtxError> {
        self.num += 1;
        self.reader
            .as_mut()
            .ok_or(PtxError::NoLoadedBuffer)?
            .read_until(b'\n', &mut self.buffer)
            .map_err(Into::into)
    }

    pub fn char_on_line(&mut self) -> Option<char> {
        if self.buffer.is_empty() {
            None
        } else {
            Some(self.buffer.remove(0).into())
        }
    }

    pub fn token_string(&mut self) -> String {
        let mut token = String::new();
        //println!("token_string w/ line = {:?}", self.line);
        while let Some(c) = self.char_on_line() {
            if c == ' ' {
                break
            } else {
                token.push(c)
            }
        }
        token
    }

    pub fn drain_buffer(&mut self) -> Result<String, PtxError> {
        String::from_utf8(
            self
                .buffer
                .drain(..)
                .collect()
        ).map(|str|
        {
            str.trim_end_matches('\n')
                .into()
        }).map_err(Into::into)
    }

    pub fn trimmed_drain_buffer(&mut self) -> Result<String, PtxError> {
        String::from_utf8(
            self
                .buffer
                .drain(..)
                .collect()
        ).map(|str|
        {
            str.trim()
                .into()
        }).map_err(Into::into)
    }
}
