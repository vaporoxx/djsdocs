use crate::data::APIData;

pub fn create(data: &APIData, source: &str) -> String {
	let base = match source {
		"stable" => "https://discord.js.org/#/docs/main/stable",
		"master" => "https://discord.js.org/#/docs/main/main",
		"collection" => "https://discord.js.org/#/docs/collection/main",
		"rpc" => "https://discord.js.org/#/docs/rpc/master",
		"commando" => "https://discord.js.org/#/docs/commando/master",
		"akairo-master" => "https://discord-akairo.github.io/#/docs/main/master",
		_ => cli::exit("encountered unknown source when parsing the URL"),
	};

	let parent = data.parent.as_ref();

	match data.internal_type.as_str() {
		"class" => format!("{}/class/{}", base, data.name),
		"typedef" => format!("{}/typedef/{}", base, data.name),
		"prop" | "method" => format!("{}/class/{}?scrollTo={}", base, parent.unwrap(), data.name),
		"event" => format!("{}/class/{}?scrollTo=e-{}", base, parent.unwrap(), data.name),
		_ => cli::exit("encountered unknown element type when parsing the URL"),
	}
}
