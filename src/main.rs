pub mod run;
use bandit::build_new_app;

fn main() {
	let config = build_new_app().unwrap();
	println!("Running Bandit file: {}", config.filename);

	let _bandit: run::Run = run::init(config.filename);
}