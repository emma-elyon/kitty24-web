use std::{
    fs::{create_dir_all, read_to_string},
    path::PathBuf,
};

use assembler::Assembler;

pub struct Compiler {}

impl Compiler {
    pub fn compile(folder: &PathBuf) -> Result<Vec<u8>, std::io::Error> {
        let src_folder = folder.join("src");
        create_dir_all(&src_folder)?;
        let source = Self::read_source_recursively(&src_folder)?;
        Assembler::assemble(&source)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))
    }

    fn read_source_recursively(folder: &PathBuf) -> Result<String, std::io::Error> {
        let mut string = String::new();
        for entry in folder
            .read_dir()
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?
            .map(|entry| entry.unwrap())
        {
            if entry.metadata().unwrap().is_dir() {
                string += &Self::read_source_recursively(&entry.path())?;
            } else {
                match entry.path().extension().unwrap().to_str().unwrap() {
                    "kittyasm" => string += &read_to_string(entry.path())?,
                    _ => {}
                }
            }
        }
        Ok(String::from(""))
    }
}
