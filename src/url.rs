use crate::data::APIData;
use crate::util;

pub fn create(data: &APIData, source: &str) -> String {
	let base = if source.starts_with("akairo") {
		"https://discord-akairo.github.io/#/docs/main/master".to_owned()
	} else {
		let main = source == "stable" || source == "master";
		format!(
			"https://discord.js.org/#/docs/{}/{}",
			if main { "main" } else { source },
			if main { source } else { "master" },
		)
	};

	let name = data.name.as_ref().unwrap();
	let parent = data.parent.as_ref();

	match data.internal_type.as_ref().unwrap().as_str() {
		"class" => format!("{}/class/{}", base, name),
		"typedef" => format!("{}/typedef/{}", base, name),
		"prop" | "method" => format!("{}/class/{}?scrollTo={}", base, parent.unwrap(), name),
		"event" => format!("{}/class/{}?scrollTo=e-{}", base, parent.unwrap(), name),
		r#type => util::exit(format!("Unexpected type encountered: {}.", r#type)),
	}
}
