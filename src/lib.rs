pub mod cli;

use cargo_metadata::TargetKind;
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
        let mut config: FileConfig = serde_json::from_reader(reader)?;
        
        // Convert relative paths to absolute paths based on config file location
        config.resolve_relative_paths(path)?;
        
        Ok(config)
    }
    
    /// Resolves relative paths in the config to absolute paths based on the config file's location
    fn resolve_relative_paths(&mut self, config_file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Get the directory containing the config file
        let config_dir = config_file_path.parent()
            .ok_or("Config file has no parent directory")?;
        
        // Resolve target_folder if it's a relative path
        let target_path = Path::new(&self.target_folder);
        if target_path.is_relative() {
            self.target_folder = config_dir.join(target_path)
                .canonicalize()
                .unwrap_or_else(|_| config_dir.join(target_path))
                .to_string_lossy()
                .to_string();
            
            // Ensure it ends with a separator
            if !self.target_folder.ends_with('/') && !self.target_folder.ends_with('\\') {
                self.target_folder.push('/');
            }
        }
        
        // Resolve SSH key path if it's relative or contains tilde
        if let Some(ref key_path) = self.key {
            if key_path.starts_with("~/") {
                // Handle tilde expansion
                if let Ok(home_dir) = std::env::var("HOME") {
                    let expanded_path = key_path.replacen("~", &home_dir, 1);
                    self.key = Some(expanded_path);
                }
            } else {
                let key_path_obj = Path::new(key_path);
                if key_path_obj.is_relative() {
                    let resolved_key = config_dir.join(key_path_obj)
                        .canonicalize()
                        .unwrap_or_else(|_| config_dir.join(key_path_obj));
                    self.key = Some(resolved_key.to_string_lossy().to_string());
                }
            }
        }
        
        Ok(())
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
        let password = cli.password.unwrap_or(file_config.password.unwrap_or_default());
        let key = cli.key.or(file_config.key.clone());
        let target_folder = cli.target_folder.unwrap_or(file_config.target_folder);
        let target = cli.target.unwrap_or(file_config.target.unwrap());
        let remote_folder = cli.remote_folder.unwrap_or(
            file_config
                .remote_folder
                .unwrap_or("/tmp/binaries/".to_string()),
        );
        let debug = cli.debug;
        
        // Auto-detect binaries if not specified
        let binaries = if cli.binaries.is_empty() {
            match detect_cargo_binaries() {
                Ok(auto_binaries) if !auto_binaries.is_empty() => {
                    println!("Auto-detected binaries: {}", auto_binaries.join(", "));
                    auto_binaries
                }
                Ok(_) => {
                    eprintln!("Warning: No binaries detected automatically. Please specify --binaries manually.");
                    cli.binaries
                }
                Err(e) => {
                    eprintln!("Warning: Failed to auto-detect binaries ({}). Please specify --binaries manually.", e);
                    cli.binaries
                }
            }
        } else {
            cli.binaries
        };
        
        let profile = file_config.profile.unwrap_or_else(|| cli.profile.clone());
        
        // Auto-detect if build is needed when not explicitly specified
        let build = if cli.build {
            true
        } else {
            // Check if we're in a cargo project and if binaries exist
            should_auto_build(&target_folder, &target, &binaries, &profile, debug)
        };

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

/// Detects binary targets from the current Cargo project using cargo metadata
pub fn detect_cargo_binaries() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let metadata = cargo_metadata::MetadataCommand::new().exec()?;
    
    // Get the root package (the current workspace member or the main package)
    let root_package = metadata.root_package()
        .ok_or("No root package found - are you in a Cargo project?")?;
    
    // Extract binary targets
    let binaries: Vec<String> = root_package
        .targets
        .iter()
        .filter_map(|target| {
            // Check if this target is a binary
            if target.kind.iter().any(|k| k == &TargetKind::Bin) {
                Some(target.name.clone())
            } else {
                None
            }
        })
        .collect();
    Ok(binaries)
}

/// Determines if an automatic build is needed based on whether binaries exist and are up-to-date
fn should_auto_build(target_folder: &str, target: &str, binaries: &[String], profile: &str, debug: bool) -> bool {
    // First, check if we're in a cargo project
    if !Path::new("Cargo.toml").exists() {
        return false; // Not a cargo project, no build needed
    }
    
    // Build the expected binary path
    let mut binary_path = std::path::PathBuf::from(target_folder);
    if !target.is_empty() {
        binary_path.push(target);
    }
    
    // Add profile directory
    if debug {
        binary_path.push("debug");
    } else {
        binary_path.push(profile);
    }
    
    // Check if any of the binaries are missing or need rebuilding
    for binary_name in binaries {
        let full_binary_path = binary_path.join(binary_name);
        
        // If binary doesn't exist, we need to build
        if !full_binary_path.exists() {
            println!("Auto-build: Binary '{}' not found at {}", binary_name, full_binary_path.display());
            return true;
        }
        
        // Check if Cargo.toml is newer than the binary (simple staleness check)
        if let (Ok(cargo_meta), Ok(binary_meta)) = (
            std::fs::metadata("Cargo.toml"),
            std::fs::metadata(&full_binary_path)
        ) {
            if let (Ok(cargo_time), Ok(binary_time)) = (
                cargo_meta.modified(),
                binary_meta.modified()
            ) {
                if cargo_time > binary_time {
                    println!("Auto-build: Binary '{}' is older than Cargo.toml", binary_name);
                    return true;
                }
            }
        }
    }
    
    // All binaries exist and appear up-to-date
    println!("Auto-build: Binaries appear up-to-date, skipping build");
    false
}
