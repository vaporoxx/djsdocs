pub enum ElementType {
	Function,
	Class,
	Typedef,
}

impl ElementType {
	pub fn as_str(&self) -> &str {
		match self {
			ElementType::Function => "function",
			ElementType::Class => "class",
			ElementType::Typedef => "typedef",
		}
	}
}

fn base(source: &str, tag: &str) -> String {
	format!("https://discord.js.org/#/docs/{}/{}", source, tag)
}

pub fn project(source: &str, tag: &str) -> String {
	base(source, tag) + "/general/welcome"
}

pub fn element(source: &str, tag: &str, name: &str, element_type: ElementType) -> String {
	base(source, tag) + "/" + element_type.as_str() + "/" + name
}
