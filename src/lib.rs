pub mod cli;

use cli::CliArgs;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

#[derive(Debug, Deserialize, Serialize)]
pub struct FileConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub key: Option<String>,
    pub target_folder: String,
    pub target: Option<String>,
    pub remote_folder: Option<String>,
    pub profile: Option<String>,
}

impl FileConfig {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    pub fn cli_overide(&mut self, cli: &CliArgs) {
        if let Some(host) = &cli.host {
            self.host = Some(host.clone());
        }
        if let Some(port) = cli.port {
            self.port = Some(port);
        }
        if let Some(username) = &cli.username {
            self.username = Some(username.clone());
        }
        if let Some(password) = &cli.password {
            self.password = Some(password.clone());
        }
        if let Some(key) = &cli.key {
            self.key = Some(key.clone());
        }
        if let Some(remote_folder) = &cli.remote_folder {
            self.remote_folder = Some(remote_folder.clone());
        }
        if !cli.profile.is_empty() {
            self.profile = Some(cli.profile.clone());
        }
        if let Some(target_folder) = &cli.target_folder {
            self.target_folder = target_folder.clone();
            self.target = None;
        }
    }

    pub fn create_empty(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub key: Option<String>,
    pub target_folder: String,
    pub target: String,
    pub remote_folder: String,
    pub debug: bool,
    pub build: bool,
    pub binaries: Vec<String>,
    pub profile: String,
}

impl Config {
    pub fn from_cli(cli: CliArgs, file_config: FileConfig) -> Self {
        let host = cli.host.unwrap_or(file_config.host.unwrap());
        let port = cli.port.unwrap_or(file_config.port.unwrap_or(22));
        let username = cli.username.unwrap_or(file_config.username.unwrap());
        let password = cli.password.unwrap_or(file_config.password.unwrap());
        let key = cli.key.or(file_config.key.clone());
        let target_folder = cli.target_folder.unwrap_or(file_config.target_folder);
        let target = cli.target.unwrap_or(file_config.target.unwrap());
        let remote_folder = cli.remote_folder.unwrap_or(
            file_config
                .remote_folder
                .unwrap_or("/tmp/binaries/".to_string()),
        );
        let debug = cli.debug;
        let binaries = cli.binaries;
        let build = cli.build;
        let profile = file_config.profile.unwrap_or_else(|| cli.profile.clone());

        Config {
            host,
            port,
            username,
            password,
            key,
            target_folder,
            target,
            remote_folder,
            debug,
            build,
            binaries,
            profile,
        }
    }
}
