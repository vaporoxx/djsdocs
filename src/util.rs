use regex::Regex;

pub fn clean(input: &str) -> String {
	let regexes = [
		Regex::new(r"`([^`]*)`").unwrap(),
		Regex::new(r"\{@link ([^}]*)\}").unwrap(),
	];

	regexes
		.iter()
		.fold(input.into(), |acc, regex| regex.replace_all(&acc, "$1").into_owned())
}
