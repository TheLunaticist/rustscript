mod function;
mod byte_crawler;
mod parser;
mod error;

use crate::function::{parse_function, ParseFunctionResult};
use crate::parser::Parser;
use crate::error::{Error, ErrorCode};

fn main() {
	let code_buf = "fn test() {}";
	
	match run_rss(code_buf) {
		RunRssResult::Finished => {
			println!("successfully executed code");
		},
		RunRssResult::Error(error) => {
			error.print();
		}
	}
}

fn run_rss(code_buf: &str) -> RunRssResult {
	let mut parser = Parser::new(code_buf);
	
	match parse_function(&mut parser) {
		ParseFunctionResult::Error(error) => {
			return RunRssResult::Error(error);
		},
		ParseFunctionResult::StartedAtBufferEnd => {
			return RunRssResult::Finished
		},
		ParseFunctionResult::NoFunction => {}
	}
	
	return RunRssResult::Error(Error::new_tell(ErrorCode::NothingFittingFound));
}

enum RunRssResult {
	Finished,
	Error(Error)
}