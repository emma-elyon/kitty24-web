use std::{
    fs::{create_dir_all, read_to_string, File},
    io::{Read, Write},
    net::TcpListener,
    path::PathBuf,
};

use compiler::Compiler;
use dialoguer::{console::Term, FuzzySelect, Select};
use serde::{Deserialize, Serialize};

const SETTINGS_PATH: &str = "settings.toml";
const KITTY24_FOLDER: &str = "Kitty24";

pub struct Console {
    settings: Settings,
    out: Term,
    _err: Term,
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
        Self {
            settings,
            out,
            _err: err,
        }
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
                Some(selection) => {
                    self.open_project_folder(projects_folder.join(&folders[selection]))?
                }
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
        opener::reveal(folder)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))
    }

    fn run_in_browser(&self, folder: &PathBuf) -> Result<(), std::io::Error> {
        let rom = Compiler::compile(folder)?;
        opener::open("http://localhost:3932/")
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?;
        // Inline http server ...(*￣０￣)ノ
        let listener = TcpListener::bind("127.0.0.1:3932").unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0u8; 4096];
                    match stream.read(&mut buffer) {
                        Ok(length) => {
                            let request = String::from_utf8_lossy(&buffer[..length]);
                            let request = request.split(' ').collect::<Vec<_>>();
                            if request.len() > 1 {
                                let path = request[1];
                                let (content, content_type): (&[u8], &str) = match path {
                                    "/audio-processor.js" => (
                                        include_bytes!("../../web/audio-processor.js"),
                                        "application/javascript; charset=utf-8",
                                    ),
                                    "/" => (
                                        include_bytes!("../../web/index.html"),
                                        "text/html; charset=utf-8",
                                    ),
                                    "/kitty24.js" => (
                                        include_bytes!("../../web/kitty24.js"),
                                        "application/javascript; charset=utf-8",
                                    ),
                                    "/kitty24.wasm" => (
                                        include_bytes!("../../web/kitty24.wasm"),
                                        "application/wasm",
                                    ),
                                    "/style.css" => (
                                        include_bytes!("../../web/style.css"),
                                        "text/css; charset=utf-8",
                                    ),
                                    "/favicon.ico" => {
                                        (include_bytes!("../../web/favicon.png"), "image/png")
                                    }
                                    "/rom.kitty24" => (&rom, "application/octet-stream"),
                                    _ => (&[], "application/octet-stream"),
                                };
                                let header = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n",
                                    content_type
                                );
                                let response = [header.as_bytes(), content].concat();
                                match stream.write(&response) {
                                    Err(error) => eprintln!("ERROR: {}", error),
                                    _ => {}
                                }
                            }
                        }
                        Err(error) => eprintln!("ERROR: {}", error),
                    }
                }
                Err(error) => eprintln!("ERROR: {}", error),
            }
        }
        loop {
            if dialoguer::Confirm::new()
                .with_prompt("Stop server?")
                .interact()
                .unwrap()
            {
                return Ok(());
            }
        }
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

    fn _write_settings(&self) {
        let settings = toml::to_string(&self.settings).unwrap();
        write!(File::create(SETTINGS_PATH).unwrap(), "{}", settings).unwrap();
    }
}
