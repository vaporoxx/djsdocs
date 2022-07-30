mod args;
mod data;
mod output;
mod url;
mod util;

use args::Args;
use clap::Parser;
use reqwest::blocking as reqwest;
use url::ElementType;

fn main() {
	let args = Args::parse();

	let source = args.source;
	let tag = args.tag;

	let url = format!(
		"https://raw.githubusercontent.com/discordjs/docs/main/{}/{}.json",
		source, tag,
	);

	let response = util::unwrap(reqwest::get(url));

	if !response.status().is_success() {
		util::exit(format!("invalid source or tag: {}/{}", source, tag));
	}

	let data = util::unwrap(response.json());

	if args.query.is_none() {
		let url = url::project(&source, &tag);
		return output::print_data(data, &url);
	}

	let query = args.query.unwrap().to_lowercase();

	if let Some(element) = data.classes.into_iter().find(|e| e.name.to_lowercase() == query) {
		let url = url::element(&source, &tag, &element.name, ElementType::Class);
		return output::print_element(element, &url, ElementType::Class);
	}

	if let Some(element) = data.typedefs.into_iter().find(|e| e.name.to_lowercase() == query) {
		let url = url::element(&source, &tag, &element.name, ElementType::Typedef);
		return output::print_element(element, &url, ElementType::Typedef);
	}

	util::exit(format!("invalid query: {}", query));
}
