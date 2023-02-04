use crate::parser::{Parser, CheckForStringResult, SkipWhitespaceResult, ParseLowercaseIdentifierResult};
use crate::error::{Error, ErrorCode, FunctionError, MiscError};

use std::string::String;

pub fn parse_function(parser: &mut Parser) -> ParseFunctionResult {	
	match parser.check_for_string("fn") {
		CheckForStringResult::Error(error) => {
			return ParseFunctionResult::Error(error)
		},
		CheckForStringResult::StartedAtBufferEnd => {
			return ParseFunctionResult::StartedAtBufferEnd
		},
		CheckForStringResult::FoundNothing => {
			return ParseFunctionResult::NoFunction
		},
		CheckForStringResult::FoundIt => {}
	}
	
	match parser.skip_whitespace() {
		SkipWhitespaceResult::Error(error) => {
			return ParseFunctionResult::Error(error)
		},
		SkipWhitespaceResult::ReachedBufferEnd => {
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::FunctionNeverEnds)))
		},
		SkipWhitespaceResult::DidIt => {}
	}
	
	let identifier;
	match parser.parse_lowercase_identifier() {
		ParseLowercaseIdentifierResult::Error(error) => {
			return ParseFunctionResult::Error(error)
		},
		ParseLowercaseIdentifierResult::ReachedBufferEnd => {
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::FunctionNeverEnds)))
		},
		ParseLowercaseIdentifierResult::FoundNothing => {
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::ExpectedIdentifier)))
		},
		ParseLowercaseIdentifierResult::GotIt(it) => {
			identifier = String::from(it);
		}
	}
	
	println!("entering low debug stage of function parsing");
	match parser.check_for_string("() {") {
		CheckForStringResult::Error(error) => {
			return ParseFunctionResult::Error(error)
		},
		CheckForStringResult::StartedAtBufferEnd => {
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::FunctionNeverEnds)))
		},
		CheckForStringResult::FoundNothing => {
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::FunctionNeverEnds)))
		},
		CheckForStringResult::FoundIt => {}
	}
	
	match parser.skip_whitespace() {
		SkipWhitespaceResult::Error(error) => {
			return ParseFunctionResult::Error(error)
		},
		SkipWhitespaceResult::ReachedBufferEnd => {
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::FunctionNeverEnds)))
		},
		SkipWhitespaceResult::DidIt => {}
	}
	
	match parser.check_for_string("}") {
		CheckForStringResult::Error(error) => {
			return ParseFunctionResult::Error(error)
		},
		CheckForStringResult::StartedAtBufferEnd => {
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::FunctionNeverEnds)))
		},
		CheckForStringResult::FoundNothing => {
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::FunctionNeverEnds)))
		},
		CheckForStringResult::FoundIt => {}
	}
	
	return ParseFunctionResult::GotFunction(identifier, Function {})
}

pub struct Function {
	
}

pub enum ParseFunctionResult {
	Error(Error),
	StartedAtBufferEnd,
	NoFunction,
	GotFunction(String, Function)
}