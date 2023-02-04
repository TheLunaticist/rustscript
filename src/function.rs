use std::string::String;

use crate::parser::{Parser, CheckForStringResult, SkipWhitespaceResult, ParseLowercaseIdentifierResult};
use crate::error::{Error, ErrorCode, FunctionError};
use crate::operation::{parse_operation, ParseOperationResult, Operation};

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
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::FunctionNeverEnds), parser.get_char_pos()))
		},
		SkipWhitespaceResult::DidIt => {}
	}
	
	let identifier;
	match parser.parse_lowercase_identifier() {
		ParseLowercaseIdentifierResult::Error(error) => {
			return ParseFunctionResult::Error(error)
		},
		ParseLowercaseIdentifierResult::ReachedBufferEnd => {
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::FunctionNeverEnds), parser.get_char_pos()))
		},
		ParseLowercaseIdentifierResult::FoundNothing => {
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::ExpectedIdentifier), parser.get_char_pos()))
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
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::FunctionNeverEnds), parser.get_char_pos()))
		},
		CheckForStringResult::FoundNothing => {
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::FunctionNeverEnds), parser.get_char_pos()))
		},
		CheckForStringResult::FoundIt => {}
	}
	println!("skipping whitespace after {{");
	match parser.skip_whitespace() {
		SkipWhitespaceResult::Error(error) => {
			return ParseFunctionResult::Error(error)
		},
		SkipWhitespaceResult::ReachedBufferEnd => {
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::FunctionNeverEnds), parser.get_char_pos()))
		},
		SkipWhitespaceResult::DidIt => {}
	}
	
	
	let mut ops: Vec<Operation> = vec!();
	
	loop {
		match parse_operation(parser) {
			ParseOperationResult::Error(error) => {
				return ParseFunctionResult::Error(error);
			},
			ParseOperationResult::StartedAtBufferEnd => {
				return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::UnexpectedBufferEnd), parser.get_char_pos()))
			},
			ParseOperationResult::NoOperation => {
				break
			},
			ParseOperationResult::GotOperation(op) => {
				ops.push(op);
			}
		}
		
		match parser.skip_whitespace() {
			SkipWhitespaceResult::Error(error) => {
				return ParseFunctionResult::Error(error)
			},
			SkipWhitespaceResult::ReachedBufferEnd => {
				return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::FunctionNeverEnds), parser.get_char_pos()))
			},
			SkipWhitespaceResult::DidIt => {}
		}
	}
	
	
	println!("checking for closing }}");
	match parser.check_for_string("}") {
		CheckForStringResult::Error(error) => {
			return ParseFunctionResult::Error(error)
		},
		CheckForStringResult::StartedAtBufferEnd => {
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::FunctionNeverEnds), parser.get_char_pos()))
		},
		CheckForStringResult::FoundNothing => {
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::Function(FunctionError::FunctionNeverEnds), parser.get_char_pos()))
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