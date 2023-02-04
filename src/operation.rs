use std::string::String;

use crate::parser::{Parser, ParseLowercaseIdentifierResult, CheckForStringResult};
use crate::error::{ErrorCode, Error, OperationError};

pub fn parse_operation(parser: &mut Parser) -> ParseOperationResult {
	//todo: check for all the operations that may use keywords first
	
	//this must be a function call
	let fun_identifier;
	match parser.parse_lowercase_identifier() {
		ParseLowercaseIdentifierResult::Error(error) => {
			return ParseOperationResult::Error(error)
		},
		ParseLowercaseIdentifierResult::ReachedBufferEnd => {
			return ParseOperationResult::Error(Error::new_tell(ErrorCode::Operation(OperationError::CallIsNotFinished), parser.get_char_pos()))
		},
		ParseLowercaseIdentifierResult::FoundNothing => {
			return ParseOperationResult::NoOperation
		},
		ParseLowercaseIdentifierResult::GotIt(it) => {
			fun_identifier = String::from(it);
		}
	}
	
	match parser.check_for_string("();") {
		CheckForStringResult::Error(error) => {
			return ParseOperationResult::Error(error)
		},
		CheckForStringResult::StartedAtBufferEnd => {
			return ParseOperationResult::StartedAtBufferEnd
		},
		CheckForStringResult::FoundNothing => {
			return ParseOperationResult::Error(Error::new_tell(ErrorCode::Operation(OperationError::CallNotClosed), parser.get_char_pos()))
		},
		CheckForStringResult::FoundIt => {}
	}
	
	return ParseOperationResult::GotOperation(Operation::Call(String::from(fun_identifier)));
}

pub enum ParseOperationResult {
	Error(Error),
	StartedAtBufferEnd,
	NoOperation,
	GotOperation(Operation)
}

pub enum Operation {
	Call(String)
}




