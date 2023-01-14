use crate::byte_crawler::{ByteCrawler, GetNextResult};
use crate::error::Error;

use std::str;

pub struct Parser<'a> {
	crawler: ByteCrawler<'a>
}

impl<'a> Parser<'a> {
	pub fn new(code_buf: &'a str) -> Self {
		Self {
			crawler: ByteCrawler::new(code_buf)
		}
	}
	
	pub fn check_for_string(&'a mut self, string: &str) -> CheckForStringResult<'a> {
		let start_pos = self.crawler.get_pos();
		let mut on_first_char = true; 
		
		for schar in string.chars() {
			match self.crawler.get_next() {
				GetNextResult::Error(error) => {
					return CheckForStringResult::Error(error)
				},
				GetNextResult::ReachedBufferEnd => {
					if on_first_char {
						self.crawler.set_pos(start_pos);
						return CheckForStringResult::StartedAtBufferEnd
					}
					self.crawler.set_pos(start_pos);
					return CheckForStringResult::FoundNothing
				},
				GetNextResult::GotChar(fchar) => {
					if schar != fchar {
						self.crawler.set_pos(start_pos);
						return CheckForStringResult::FoundNothing
					}
					
					on_first_char = false;
				}
			}
		}
		
		return CheckForStringResult::FoundIt
	}
	
	pub fn skip_whitespace(&'a mut self) -> SkipWhitespaceResult<'a> {
		loop {
			let last_pos = self.crawler.get_pos();
			match self.crawler.get_next() {
				GetNextResult::Error(error) => {
					return SkipWhitespaceResult::Error(error)
				},
				GetNextResult::ReachedBufferEnd => {
					self.crawler.set_pos(last_pos);
					return SkipWhitespaceResult::ReachedBufferEnd
				},
				GetNextResult::GotChar(ch) => {
					if ch != ' ' && ch != '\n' && ch != '\r' {
						self.crawler.set_pos(last_pos);
						return SkipWhitespaceResult::DidIt
					}
				}
			}
		}
	}
	
	pub fn parse_function_identifier(&mut self) -> ParseFunctionIdentifierResult {
		let start_pos = self.crawler.get_pos();
		let mut num_chars = 0;
		loop {
			let last_pos = self.crawler.get_pos();
			match self.crawler.get_next() {
				GetNextResult::Error(error) => {
					return ParseFunctionIdentifierResult::Error(error)
				},
				GetNextResult::ReachedBufferEnd => {
					self.crawler.set_pos(last_pos);
					return ParseFunctionIdentifierResult::ReachedBufferEnd
				},
				GetNextResult::GotChar(ch) => {
					match u32::from(ch) {
						97..=122 | 95 => {
							num_chars += 1;
						},
						_ => {
							self.crawler.set_pos(last_pos);
							if num_chars == 0 {
								return ParseFunctionIdentifierResult::FoundNothing
							}
							unsafe {
								return ParseFunctionIdentifierResult::GotIt(str::from_utf8_unchecked(&self.crawler.get_buf()[start_pos..start_pos + num_chars]))
							}
						}
					}
				}
			}
		}
	}
}

pub enum CheckForStringResult<'a> {
	Error(Error<'a>),
	StartedAtBufferEnd,
	FoundNothing,
	FoundIt
}

pub enum SkipWhitespaceResult<'a> {
	Error(Error<'a>),
	ReachedBufferEnd,
	DidIt
}

pub enum ParseFunctionIdentifierResult<'a> {
	Error(Error<'a>),
	GotIt(&'a str),
	ReachedBufferEnd,
	FoundNothing
}