use std::fs::create_dir_all;

use dirs;

fn main() {
	let audio_processor_js = include_str!("../../web/audio-processor.js");
	let index_html = include_str!("../../web/index.html");
	let kitty24_js = include_str!("../../web/kitty24.js");
	let kitty24_wasm = include_bytes!("../../web/kitty24.wasm");
	let style_css = include_str!("../../web/style.css");
	let documents = dirs::document_dir().unwrap();
	let root_folder = documents.join("Kitty24");
	let projects_folder = root_folder.join("projects");
	let saves_folder = root_folder.join("saves");
	match projects_folder.try_exists() {
		Ok(false) => create_dir_all(projects_folder.clone()).expect("Could not create projects folder."),
		Err(error) => panic!("{}", error),
		_ => {}
	}
	match projects_folder.read_dir() {
		Ok(directory) => {
			for entry in directory {
				if let Ok(entry) = entry {
					println!("{}", entry.file_name().to_str().unwrap_or("INVALID"));
				}
			}
		},
		Err(error) => panic!("{}", error),
	}
}
