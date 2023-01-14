mod function;
mod byte_crawler;
mod parser;
mod error;

use crate::function::parse_function;
use crate::parser::Parser;

fn main() {
	let code_buf = "test();";
	
	match run_rss(code_buf) {
		RunRssResult::Finished => {
			println!("successfully executed code");
		}
	}
}

fn run_rss(code_buf: &str) -> RunRssResult {
	let parser = Parser::new(code_buf);
	parse_function();
	RunRssResult::Finished
}

enum RunRssResult {
	Finished
}