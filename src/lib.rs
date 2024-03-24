use clap::{ Error, Command, Arg };


pub struct Config {
	pub filename: String,
}

pub fn build_new_app() -> Result<Config, Error> {
	let config = Command::new("bandit")
		.version("0.1.0")
		.author("dvnc0")
		.about("Bandit automation script runner")
		.arg(Arg::new("filename")
			.value_name("FILENAME")
			.help("The Bandit file to run")
			.required(true)
		).get_matches();

	let filename: String = config.get_one::<String>("filename").unwrap().to_string();
	Ok(Config { filename })
}