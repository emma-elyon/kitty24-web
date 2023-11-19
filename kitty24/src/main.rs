use std::{
	fs::{create_dir_all, read_to_string, File},
	io::Write,
	path::{Path, PathBuf},
	process::exit,
};

use serde::{Deserialize, Serialize};

use dialoguer::{self, Select};
use dirs::{self, document_dir};

enum State {
	Start,
	Projects,
	RunRom,
	DebugRom,
	Settings,
	Exit,
}

#[derive(Serialize, Deserialize)]
struct Settings {
	path: Option<PathBuf>,
	target: Option<Target>,
}

#[derive(Serialize, Deserialize)]
enum Target {
	Web,
	Native,
}

fn main() {
	let kitty24 = dialoguer::console::style("Kitty24")
		.bright()
		.bold()
		.magenta();

	// Initialize or load settings.
	let mut settings = match read_to_string("settings.toml") {
		Ok(settings) => parse_settings(Some(settings)),
		_ => parse_settings::<String>(None),
	};

	// Write settings to file system.
	// TODO: Only write when settings change.
	let settings_string = toml::to_string(&settings).expect("Could not write settings.toml");
	write!(
		File::create("settings.toml").unwrap(),
		"{}",
		settings_string
	)
	.unwrap();

	let mut state = State::Start;

	let _audio_processor_js = include_str!("../../web/audio-processor.js");
	let _index_html = include_str!("../../web/index.html");
	let _kitty24_js = include_str!("../../web/kitty24.js");
	let _kitty24_wasm = include_bytes!("../../web/kitty24.wasm");
	let _style_css = include_str!("../../web/style.css");

	loop {
		dialoguer::console::Term::stdout().clear_screen().unwrap();
		println!("{} 0.1.0", kitty24);
		state = match state {
			State::Start => prompt_start(),
			State::Projects => prompt_projects(&settings),
			State::RunRom => prompt_run_rom(),
			State::DebugRom => prompt_debug_rom(),
			State::Settings => prompt_settings(&mut settings),
			State::Exit => exit(0),
		};
	}
}

fn parse_settings<S>(settings: Option<S>) -> Settings
where
	S: Into<String>,
{
	let settings: String = settings.map(|s| s.into()).unwrap_or("".into());
	let mut settings: Settings = toml::from_str(&settings).unwrap();

	settings.path = match settings.path {
		Some(path) => Some(path),
		None => {
			let input = dialoguer::Input::<String>::new()
				.with_prompt("path")
				.with_initial_text(document_dir().unwrap().join("Kitty24").to_str().unwrap())
				.interact_text()
				.unwrap();
			Some(Path::new(&input).to_path_buf())
		}
	};

	settings.target = match settings.target {
		Some(target) => Some(target),
		None => {
			let items = ["web", "native"];
			let index = dialoguer::Select::new()
				.default(0)
				.with_prompt("target")
				.items(&items)
				.interact()
				.unwrap();
			match index {
				0 => Some(Target::Web),
				1 => Some(Target::Native),
				_ => unreachable!(),
			}
		}
	};

	settings
}

fn prompt_start() -> State {
	let selection = dialoguer::Select::new()
		.default(0)
		.item("Open project")
		.item("Open ROM")
		.item("Debug ROM")
		.item("Settings")
		.interact_opt()
		.unwrap();

	match selection {
		Some(0) => State::Projects,
		Some(1) => State::RunRom,
		Some(2) => State::DebugRom,
		Some(3) => State::Settings,
		_ => State::Exit,
	}
}

fn prompt_projects(settings: &Settings) -> State {
	let projects_folder = settings.path.clone().unwrap().join("projects");

	// Create projects folder if it does not exist.
	match projects_folder.try_exists() {
		Ok(false) => {
			create_dir_all(projects_folder.clone()).expect("Could not create projects folder.")
		}
		Err(error) => panic!("{}", error),
		_ => {}
	}

	// Prompt project and command.
	match projects_folder.read_dir() {
		Ok(directory) => {
			let entries: Vec<_> = directory
				.map(|entry| {
					entry
						.unwrap()
						.file_name()
						.to_str()
						.unwrap_or("INVALID NAME")
						.to_string()
				})
				.collect();
			let entries = entries.as_slice();
			let _project_selection = dialoguer::FuzzySelect::new()
				.default(0)
				.with_prompt("Project")
				.items(entries)
				.interact()
				.unwrap();
			let _command_selection = dialoguer::Select::new()
				.default(0)
				.with_prompt("Command")
				.items(&["build", "debug", "run", "open"])
				.interact()
				.unwrap();
		}
		Err(error) => panic!("{}", error),
	}

	State::Start
}

fn prompt_run_rom() -> State {
	State::Start
}

fn prompt_debug_rom() -> State {
	State::Start
}

fn prompt_settings(settings: &mut Settings) -> State {
	let path = settings.path.clone().unwrap();
	let path = path.to_str().unwrap();
	let path = format!("Path: {}", path);
	let target = match settings.target.as_ref().unwrap() {
		Target::Web => "Web",
		Target::Native => "Native",
	};
	let target = format!("Target: {}", target);
	let items = [path, target];
	let selection = Select::new()
		.default(0)
		.items(&items)
		.interact_opt()
		.unwrap();

	match selection {
		Some(0) => {}
		Some(1) => {
			settings.target = match settings.target.as_ref().unwrap() {
				Target::Web => Some(Target::Native),
				Target::Native => Some(Target::Web),
			}
		}
		_ => return State::Start,
	}

	// Write settings to file system.
	// TODO: Only write when settings change.
	let settings_string = toml::to_string(&settings).expect("Could not write settings.toml");
	write!(
		File::create("settings.toml").unwrap(),
		"{}",
		settings_string
	)
	.unwrap();

	State::Settings
}
