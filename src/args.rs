use std::collections::HashMap;
use std::env;

pub struct Args {
	pub options: HashMap<String, String>,
	pub positionals: Vec<String>,
}

fn parse_option(option: &str, options: &mut HashMap<String, String>) {
	let mut parts = option.splitn(2, '=');

	let name = parts.next().unwrap();
	let value = parts.next().unwrap_or_default();

	if !name.is_empty() {
		options.insert(name.into(), value.into());
	}
}

pub fn parse() -> Args {
	let mut options = HashMap::new();
	let mut positionals = Vec::new();

	for arg in env::args().skip(1) {
		if let Some(option) = arg.strip_prefix("--") {
			parse_option(option, &mut options);
		} else {
			positionals.push(arg);
		}
	}

	Args { options, positionals }
}
