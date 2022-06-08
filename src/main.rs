mod args;
mod data;
mod output;
mod url;
mod util;

use reqwest::blocking as reqwest;
use url::ElementType;

fn main() {
	let args = args::parse();

	let source = args.options.get("src").map_or("discord.js", String::as_str);
	let tag = args.options.get("tag").map_or("main", String::as_str);

	let url = format!(
		"https://raw.githubusercontent.com/discordjs/docs/main/{}/{}.json",
		source, tag,
	);

	let response = util::unwrap(reqwest::get(url));

	if !response.status().is_success() {
		util::exit(format!("invalid source or tag: {}/{}", source, tag));
	}

	let data = util::unwrap(response.json());

	if args.positionals.is_empty() {
		let url = url::project(source, tag);
		return output::print_data(data, &url);
	}

	let query = args.positionals[0].to_lowercase();

	if let Some(element) = data.classes.into_iter().find(|e| e.name.to_lowercase() == query) {
		let url = url::element(source, tag, &element.name, ElementType::Class);
		return output::print_element(element, &url, ElementType::Class);
	}

	if let Some(element) = data.typedefs.into_iter().find(|e| e.name.to_lowercase() == query) {
		let url = url::element(source, tag, &element.name, ElementType::Typedef);
		return output::print_element(element, &url, ElementType::Typedef);
	}

	util::exit(format!("invalid query: {}", query));
}
