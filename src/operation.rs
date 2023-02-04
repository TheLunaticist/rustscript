use crate::parser::{Parser, ParseLowercaseIdentifierResult, CheckForStringResult};
use crate::error::{ErrorCode, Error};

pub fn parse_operation(parser: &mut Parser) -> ParseOperationResult {
	//todo: check for all the operations that may use keywords first
	
	//this must be a function call
	let identifier;
	match parser.parse_lowercase_identifier() {
		ParseLowercaseIdentifierResult::Error(error) => {
			return ParseOperationResult::Error(error)
		},
		ParseLowercaseIdentifierResult::ReachedBufferEnd => {
			return ParseOperationResult::Error(Error::new_tell(ErrorCode::FunctionNeverEnds))
		},
		ParseLowercaseIdentifierResult::FoundNothing => {
			return ParseOperationResult::Error(Error::new_tell(ErrorCode::ExpectedIdentifier))
		},
		ParseLowercaseIdentifierResult::GotIt(it) => {
			identifier = String::from(it);
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
			return ParseOperationResult::Error(Error::new_tell())
		},
		CheckForStringResult::FoundIt => {}
	}
	
	return ParseOperationResult::Error(Error::new_tell(ErrorCode::NotImplemented))
}

pub enum ParseOperationResult {
	Error(Error),
	StartedAtBufferEnd,
	
}




