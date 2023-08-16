use railwind::{parse_to_string, Source, SourceOptions};
use serde::de::DeserializeOwned;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

pub fn get_files_in_dir(dir: &str) -> std::io::Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let extension = path.extension().and_then(|s| s.to_str());
        if path.is_file() && extension == Some("html") {
            file_paths.push(path);
        }
    }

    Ok(file_paths)
}

pub fn build_css_classes(paths: &Vec<PathBuf>) -> String {
    let mut warnings = vec![];
    let sources = Source::Files(
        paths
            .iter()
            .map(|pb| SourceOptions {
                input: &pb,
                option: railwind::CollectionOptions::Html,
            })
            .collect(),
    );

    parse_to_string(sources, true, &mut warnings)
}

pub fn parse_toml<T: DeserializeOwned>(path: &str) -> Result<T, Box<dyn Error>> {
    let mut file = fs::File::open(Path::new(path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let value: T = toml::from_str(&contents)?;

    Ok(value)
}
