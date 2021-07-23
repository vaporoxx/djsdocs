use crate::{url, util};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ParameterData {
	pub name: String,
	pub description: String,
	pub r#type: String,
}

#[derive(Deserialize)]
pub struct ReturnData {
	pub r#type: String,
}

#[derive(Deserialize)]
pub struct APIData {
	pub message: Option<String>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub internal_type: Option<String>,
	pub parent: Option<String>,
	pub props: Option<Vec<String>>,
	pub methods: Option<Vec<String>>,
	pub events: Option<Vec<String>>,
	pub r#type: Option<String>,
	pub params: Option<Vec<ParameterData>>,
	pub returns: Option<ReturnData>,
}

pub struct ParsedData {
	pub url: String,
	pub name: String,
	pub description: String,
	pub internal_type: String,
	pub parent: Option<String>,
	pub props: Option<Vec<String>>,
	pub methods: Option<Vec<String>>,
	pub events: Option<Vec<String>>,
	pub r#type: Option<String>,
	pub params: Option<Vec<ParameterData>>,
	pub returns: Option<ReturnData>,
}

pub fn parse(data: APIData, source: &str) -> ParsedData {
	if let Some(message) = data.message {
		util::exit(message);
	}

	ParsedData {
		url: url::create(&data, source),
		name: data.name.unwrap(),
		description: data.description.unwrap(),
		internal_type: data.internal_type.unwrap(),
		parent: data.parent,
		props: data.props,
		methods: data.methods,
		events: data.events,
		r#type: data.r#type,
		params: data.params,
		returns: data.returns,
	}
}
