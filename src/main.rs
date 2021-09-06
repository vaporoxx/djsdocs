mod data;
mod output;
mod url;
mod util;

use cli::Args;
use data::APIResponse;
use reqwest::blocking as reqwest;

fn main() {
	let args = Args::parse();

	if args.positionals.is_empty() {
		cli::exit("no query provided")
	}

	let source = args.options.get("src").map_or("stable", |e| e);
	let force = args.options.contains_key("force");

	let url = format!(
		"https://djsdocs.sorta.moe/v2?src={}&force={}&q={}",
		source,
		force,
		args.positionals.join(".").replace('#', "."),
	);

	let response = cli::unwrap(reqwest::get(url));
	let parsed = cli::unwrap(response.json::<APIResponse>());

	let data = data::parse(parsed);
	let url = url::create(&data, source);
	let compact = args.options.contains_key("compact");

	output::print(data, &url, compact);
}
