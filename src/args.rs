use crate::util;
use std::collections::HashMap;
use std::env;

pub struct Args {
	pub options: HashMap<String, String>,
	pub query: String,
}

pub fn parse() -> Args {
	let args = env::args().skip(1);
	let (options, query) = args.partition::<Vec<_>, _>(|e| e.starts_with("--"));

	if query.is_empty() {
		util::exit("No query specified.");
	}

	let mut option_map = HashMap::new();

	for option in options {
		let mut parts = option[2..].split('=');

		let name = parts.next().unwrap().to_owned();
		let value = parts.next().unwrap_or_default().to_owned();

		if !name.is_empty() {
			option_map.insert(name, value);
		}
	}

	Args {
		options: option_map,
		query: query.iter().map(|e| e.trim()).collect::<Vec<_>>().join("."),
	}
}
