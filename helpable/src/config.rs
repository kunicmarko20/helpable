use config::Config as InnerConfig;
use config::*;
use std::collections::HashMap;
use std::fs::{create_dir_all, File as StdFile, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub struct Config {
    config: InnerConfig,
    config_path: PathBuf,
}

impl Config {
    const DIRECTORY: &'static str = "helpable";
    const FILE_PATH: &'static str = "helpable/Config.toml";

    pub fn new() -> Self {
        let mut inner_config = InnerConfig::default();
        let mut config_path = dirs::data_local_dir().unwrap();

        Self::create_config_directory(config_path.clone());

        config_path.push(Self::FILE_PATH);
        if !config_path.exists() {
            Self::create_config_file(config_path.clone());
        }

        inner_config
            .merge(File::from(config_path.clone()))
            .expect("Unable to merge config.");

        Config {
            config: inner_config,
            config_path,
        }
    }

    fn create_config_directory(mut config_path: PathBuf) {
        config_path.push(Self::DIRECTORY);
        create_dir_all(config_path).expect("Unable to create a directory.");
    }

    fn create_config_file(config_file_path: PathBuf) {
        StdFile::create(config_file_path).expect("Unable to create config file.");
    }

    pub fn list(&mut self) -> HashMap<String, Value> {
        self.config.collect().expect("Unable to collect config.")
    }

    pub fn get(&mut self, key: &str) -> String {
        let value = self.config.get_str(key);

        if value.is_err() {
            return self.ask_for_value(key);
        }

        value.unwrap()
    }

    pub fn ask_for_value(&mut self, key: &str) -> String {
        println!(
            "Configuration value is missing for \"{}\", please enter it:",
            key
        );

        let value: String;
        scan!("{}", value);

        self.set(key, value.clone());
        self.save();

        value
    }

    pub fn set<T>(&mut self, key: &str, value: T)
    where
        T: Into<Value>,
    {
        self.config
            .set(key, value)
            .unwrap_or_else(|_| panic!("Unable to set {}.", key));
    }

    pub fn save(&mut self) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.config_path)
        {
            file.set_len(0).expect("Unable to truncate file.");
            for (key, value) in self.config.collect().expect("Unable to collect config.") {
                file.write_all(
                    format!(
                        "{}=\"{}\"\n",
                        key,
                        value
                            .into_str()
                            .expect("Unable to convert config value to String.")
                    )
                    .as_bytes(),
                )
                .expect("Unable to write to Config file.");
            }
        }
    }
}
