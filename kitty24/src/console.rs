use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write,
    path::PathBuf,
};

use dialoguer::{console::Term, Select, FuzzySelect};
use serde::{Deserialize, Serialize};

const SETTINGS_PATH: &str = "settings.toml";
const KITTY24_FOLDER: &str = "Kitty24";

pub struct Console {
    settings: Settings,
    out: Term,
    err: Term,
}

#[derive(Serialize, Deserialize)]
struct Settings {
    path: PathBuf,
    target: Target,
}

#[derive(Serialize, Deserialize)]
enum Target {
    Web,
    Native,
}

impl Console {
    pub fn new() -> Self {
        let settings = Self::read_settings();
        let out = Term::stdout();
        let err = Term::stderr();
        Self { settings, out, err }
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        self.start()
    }

    fn start(&mut self) -> Result<(), std::io::Error> {
        let title = dialoguer::console::style("Kitty24")
            .bright()
            .bold()
            .magenta();
        loop {
            self.out.clear_screen()?;
            println!("{} 0.1.0", title);
            let selection = Select::new()
				.default(0)
                .item("Open project...")
                .item("Run ROM...")
                .item("Debug ROM...")
                .item("Settings...")
                .interact_opt()
                .unwrap();
            match selection {
                Some(0) => self.open_project()?,
                Some(1) => self.run_rom()?,
                Some(2) => self.debug_rom()?,
                Some(3) => self.settings()?,
                _ => return Ok(()),
            }
        }
    }

    fn open_project(&mut self) -> Result<(), std::io::Error> {
        let projects_folder = self.settings.path.join("projects");

        // Create projects folder if it does not exist.
        create_dir_all(projects_folder.clone()).unwrap();

        loop {
            self.out.clear_screen()?;
            println!("Open project (type to search)...");
            let folders = projects_folder
                .read_dir()?
                .map(|entry| entry.unwrap())
                .filter(|entry| entry.metadata().unwrap().is_dir())
                .map(|entry| entry.file_name().to_str().unwrap().to_string())
                .collect::<Vec<_>>();
			let selection = FuzzySelect::new()
				.default(0)
				.items(folders.as_slice())
				.interact_opt()
				.unwrap();
			match selection {
				Some(selection) => self.open_project_folder(projects_folder.join(&folders[selection]))?,
				None => return Ok(()),
			}
        }
    }

	fn open_project_folder(&mut self, folder: PathBuf) -> Result<(), std::io::Error> {
        let title = dialoguer::console::style(folder.file_name().unwrap().to_str().unwrap())
            .bright()
            .bold();
		let target = match self.settings.target {
			Target::Web => "in browser",
			Target::Native => "as native application",
		};
		loop {
            self.out.clear_screen()?;
            println!("{}", title);
			let selection = Select::new()
				.default(0)
				.item("Open folder in explorer")
				.item("Build project")
				.item("Debug in browser")
				.item(format!("Run {}", target))
				.interact_opt()
				.unwrap();
			match selection {
				Some(0) => self.open_folder_in_explorer(&folder)?,
				Some(1) => todo!(),
				Some(2) => todo!(),
				Some(3) => match self.settings.target {
					Target::Web => self.run_in_browser(&folder)?,
					Target::Native => todo!(),
				},
				_ => return Ok(()),
			}
		}
	}

	fn open_folder_in_explorer(&self, folder: &PathBuf) -> Result<(), std::io::Error> {
		opener::reveal(folder).map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))
	}

	fn run_in_browser(&self, folder: &PathBuf) -> Result<(), std::io::Error> {
		todo!();
	}

    fn run_rom(&mut self) -> Result<(), std::io::Error> {
        todo!();
    }

    fn debug_rom(&mut self) -> Result<(), std::io::Error> {
        todo!();
    }

    fn settings(&mut self) -> Result<(), std::io::Error> {
        todo!();
    }

    fn read_settings() -> Settings {
        let settings = read_to_string(SETTINGS_PATH);
        match settings {
            Ok(settings) => toml::from_str(&settings).unwrap(),
            Err(_) => {
                let path = dirs::document_dir().unwrap().join(KITTY24_FOLDER);
                let target = Target::Web;
                Settings { path, target }
            }
        }
    }

    fn write_settings(&self) {
        let settings = toml::to_string(&self.settings).unwrap();
        write!(File::create(SETTINGS_PATH).unwrap(), "{}", settings).unwrap();
    }
}
