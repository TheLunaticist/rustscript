use crate::parser::{Parser, CheckForStringResult, SkipWhitespaceResult, ParseFunctionIdentifierResult};
use crate::error::{Error, ErrorCode};

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
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::FunctionNeverEnds))
		},
		SkipWhitespaceResult::DidIt => {}
	}
	
	let identifier;
	match parser.parse_function_identifier() {
		ParseFunctionIdentifierResult::Error(error) => {
			return ParseFunctionResult::Error(error)
		},
		ParseFunctionIdentifierResult::ReachedBufferEnd => {
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::FunctionNeverEnds))
		},
		ParseFunctionIdentifierResult::FoundNothing => {
			return ParseFunctionResult::Error(Error::new_tell(ErrorCode::ExpectedIdentifier))
		},
		ParseFunctionIdentifierResult::GotIt(it) => {
			identifier = it;
		}
	}
	
	println!("The function name is: {}", identifier);
	
	return ParseFunctionResult::Error(Error::new_tell(ErrorCode::NotImplemented))
}

pub enum ParseFunctionResult {
	Error(Error),
	StartedAtBufferEnd,
	NoFunction
}