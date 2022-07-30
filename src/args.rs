use clap::Parser;

#[derive(Parser)]
#[clap(about)]
pub struct Args {
	#[clap(help = "The search query")]
	pub query: Option<String>,

	#[clap(short, long, help = "The docs source", default_value = "discord.js")]
	pub source: String,

	#[clap(short, long, help = "The docs tag", default_value = "main")]
	pub tag: String,
}
