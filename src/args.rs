use clap::Parser;

#[derive(Parser)]
#[command(about)]
pub struct Args {
	#[arg(help = "The search query")]
	pub query: Option<String>,

	#[arg(short, long, help = "The docs source", default_value = "discord.js")]
	pub source: String,

	#[arg(short, long, help = "The docs tag", default_value = "main")]
	pub tag: String,
}
