use std::fs::File;
use std::io::Read;
use enigo::{ Enigo, KeyboardControllable };

pub struct Run {
	pub filename: String,
	comments: Vec<String>,
}

impl Run {

	pub fn new(&mut self) { 
		if !std::path::Path::new(&self.filename).exists() {
			panic!("File does not exist");
		}
		
		if !std::path::Path::new(&self.filename).is_file() {
			panic!("File is not a file");
		}

		if !self.filename.ends_with(".bandit") {
			panic!("File is not a bandit file");
		}

		self.comments = vec!["#".to_string(), "//".to_string(), "COMM".to_string()];
	}

	// This is a WIP/POC needs to be cleaned up
	pub fn process_file(&self){
		let mut file = File::open(&self.filename).unwrap();
		let mut contents = String::new();
		
		file.read_to_string(&mut contents).unwrap();
		let lines: Vec<&str> = contents.split("\n").collect();

		for line in lines {
			// ignore comments
			if self.comments.iter().any(|comment| line.starts_with(comment)) {
				continue;
			}

			// type strings
			if line.starts_with("TYPE") {
				let line: Vec<&str> = line.splitn(2, " ").collect();
				Enigo::new().key_sequence(line[1]);
				continue;
			}

			// send keys
			if line.starts_with("SEND") {
				let line: Vec<&str> = line.splitn(2, " ").collect();
				
				// probably need to break line[1] up more for things like SHIFT+COMMAND+T

				match line[1].to_string().to_uppercase().as_str() {
					"ENTER" => Enigo::new().key_click(enigo::Key::Return),
					"SPACE" => Enigo::new().key_click(enigo::Key::Space),
					"TAB" => Enigo::new().key_click(enigo::Key::Tab),
					"ESC" => Enigo::new().key_click(enigo::Key::Escape),
					"UP" => Enigo::new().key_click(enigo::Key::UpArrow),
					"DOWN" => Enigo::new().key_click(enigo::Key::DownArrow),
					"LEFT" => Enigo::new().key_click(enigo::Key::LeftArrow),
					"RIGHT" => Enigo::new().key_click(enigo::Key::RightArrow),
					"COMMAND" => Enigo::new().key_click(enigo::Key::Meta),
					_ => Enigo::new().key_sequence(line[1]),
				}
				
				continue;
			
			}
		}
	}
}

pub fn init(filename: String) -> Run {
	let mut run: Run = Run { filename, comments: vec![] };
	run.new();
	run.process_file();
	run
}
