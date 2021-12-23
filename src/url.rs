use crate::data::ElementData;
use std::fmt::Display;

pub enum URLError {
	UnknownElementType,
	UnknownSource,
}

impl Display for URLError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let error = match self {
			URLError::UnknownElementType => "unknown element type",
			URLError::UnknownSource => "unknown source",
		};

		write!(f, "encountered {} while parsing the url", error)
	}
}

fn base_url(source: &str) -> Option<&'static str> {
	match source {
		"stable" => Some("https://discord.js.org/#/docs/main/stable"),
		"master" => Some("https://discord.js.org/#/docs/main/main"),
		"collection" => Some("https://discord.js.org/#/docs/collection/main"),
		"rpc" => Some("https://discord.js.org/#/docs/rpc/master"),
		"commando" => Some("https://discord.js.org/#/docs/commando/master"),
		"akairo-master" => Some("https://discord-akairo.github.io/#/docs/main/master"),
		_ => None,
	}
}

pub fn parse_url(data: &ElementData, source: &str) -> Result<String, URLError> {
	let base = base_url(source).ok_or(URLError::UnknownSource)?;

	let parent = data.parent.as_ref();

	match data.internal_type.as_str() {
		"class" => Ok(format!("{}/class/{}", base, data.name)),
		"typedef" => Ok(format!("{}/typedef/{}", base, data.name)),
		"prop" | "method" => Ok(format!("{}/class/{}?scrollTo={}", base, parent.unwrap(), data.name)),
		"event" => Ok(format!("{}/class/{}?scrollTo=e-{}", base, parent.unwrap(), data.name)),
		_ => Err(URLError::UnknownElementType),
	}
}
