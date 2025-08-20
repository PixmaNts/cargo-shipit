use cargo_shipit::{cli::CliArgs, Config, FileConfig};
use clap::Parser;
use ssh2::Session;
use std::{
    env,
    fs::File,
    io::{self, prelude::*},
    net::TcpStream,
    os::unix::fs::MetadataExt,
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Handle cargo subcommand integration - skip the first argument if it's "shipit"
    let args: Vec<String> = std::env::args().collect();
    let filtered_args: Vec<String> = if args.len() > 1 && args[1] == "shipit" {
        // Called as "cargo shipit" - skip the "shipit" argument
        let mut new_args = vec![args[0].clone()];
        new_args.extend_from_slice(&args[2..]);
        new_args
    } else {
        // Called directly as "cargo-shipit"
        args
    };

    let cli_args = CliArgs::parse_from(filtered_args);
    if cli_args.init.is_some() {
        let mut config = FileConfig {
            host: Some("embedded-linux-target".to_string()),
            port: Some(22),
            username: Some("root".to_string()),
            password: Some("password".to_string()),
            key: None,
            target_folder: String::from(
                env::current_dir()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default(),
            ) + "/target/", // current directory
            target: Some("armv7-unknown-linux-gnueabihf".to_string()),
            remote_folder: Some("/tmp/binaries/".to_string()),
            profile: Some("release".to_string()),
        };
        // Apply CLI overrides to the default config
        config.cli_overide(&cli_args);
        config.create_empty(Path::new(cli_args.init.unwrap().as_str()))?;
        return Ok(());
    }

    let config = FileConfig::from_file(Path::new(cli_args.config.as_str()))
        .unwrap_or_else(|e| {
            eprintln!("Failed to read configuration file '{}': {}", cli_args.config, e);
            eprintln!("Hint: Create a config file with: cargo shipit --init shipit.json");
            std::process::exit(1);
        });

    let config = Config::from_cli(cli_args, config);

    println!("{config:?}");

    if config.build {
        // Build the project
        let mut build_cmd = std::process::Command::new("cargo");
        build_cmd.arg("build").arg("--target").arg(&config.target);

        // Add profile argument if not debug
        if !config.debug {
            if config.profile == "release" {
                build_cmd.arg("--release");
            } else {
                build_cmd.arg(format!("--profile={}", config.profile));
            }
        }

        let cmd = build_cmd.status().expect("Failed to build the project");
        if !cmd.success() {
            panic!("Failed to build the project");
        }
    }

    let remote_server = format!("{}:{}", config.host, config.port);
    let username = &config.username;
    let password = &config.password;

    // Connect to the remote server
    let tcp = TcpStream::connect(remote_server).expect("Failed to connect to remote server");
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session
        .handshake()
        .expect("Failed to perform SSH handshake");

    // Authenticate with the remote server
    if let Some(key) = &config.key {
        let key_path = Path::new(key);
        let public_key_path_str = format!("{key}.pub");
        let public_key_path = Path::new(&public_key_path_str);
        session
            .userauth_pubkey_file(username, Some(public_key_path), key_path, Some(password))
            .expect("Failed to authenticate using SSH key");
    } else {
        session
            .userauth_password(username, password)
            .expect("Failed to authenticate");
    }

    // Check authentication
    if !session.authenticated() {
        panic!("Failed to authenticate with the remote server");
    }

    let remote_path = config.remote_folder.clone();
    // Check if the remote directory exists
    if !remote_directory_exists(&session, &remote_path)? {
        // Create the remote directory using sudo and the provided password.
        println!("Remote directory does not exist");
        // Ask the user if they want to create the directory
        let mut input = String::new();
        print!("Directory {remote_path} does not exist. Do you want to create it? (yes/no): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        if input == "yes" {
            // Create the remote directory using sudo
            create_remote_directory(&session, &remote_path, username, password)?;
        } else {
            return Err(format!("Directory {remote_path} not created. Aborting.").into());
        }
    }

    // Upload files
    let target_path = build_target_path(&config);
    for binarie in config.binaries.iter() {
        let source = target_path.clone() + binarie;
        let source: &Path = Path::new(&source);
        let destination = remote_path.clone() + binarie;
        println!("Uploading {} to {}", source.display(), destination);
        scp_upload(&session, source, &destination).expect("Failed to perform SCP upload");
    }
    Ok(())
}

fn scp_upload(
    session: &Session,
    local_path: &Path,
    remote_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Open the local file
    let mut local_file = File::open(local_path)?;

    // Get the file metadata to obtain the file size
    let file_size = local_file.metadata()?.size();
    match file_size {
        0..=1000 => println!("Uploading {file_size} bytes"),
        1001..=1000000 => println!("Uploading {:.4} KB", file_size as f64 / 1000.0),
        _ => println!("Uploading {:.4} MB", file_size as f64 / 1000000.0),
    }

    // Initiate the SCP upload with executable permissions
    let mut remote_file = session.scp_send(Path::new(remote_path), 0o755, file_size, None)?;

    // Read the local file and write to the remote file
    let mut buffer = Vec::new();
    local_file.read_to_end(&mut buffer)?;
    println!("Sending file...");
    remote_file.write_all(&buffer)?;
    remote_file.send_eof()?;
    remote_file.wait_eof()?;
    remote_file.close()?;
    remote_file.wait_close()?;

    Ok(())
}

fn remote_directory_exists(
    session: &Session,
    dir: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut channel = session.channel_session()?;
    channel.exec(&format!("test -d {dir}"))?;
    channel.send_eof()?;
    channel.wait_eof()?;
    channel.close()?;
    channel.wait_close()?;
    let exit_status = channel.exit_status()?;
    Ok(exit_status == 0)
}

fn create_remote_directory(
    session: &Session,
    dir: &str,
    user: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let command = format!("echo {password} | sudo -S mkdir -p {dir}");
    let chown_command = format!("echo {password} | sudo -S chown -R {user}:{user} {dir}");

    let mut channel = session.channel_session()?;
    channel.exec(&command)?;
    let exit_status = channel.exit_status()?;
    if exit_status != 0 {
        return Err(format!("Failed to create directory with exit status: {exit_status}").into());
    }

    let mut channel = session.channel_session()?;
    channel.exec(&chown_command)?;
    let exit_status = channel.exit_status()?;
    if exit_status != 0 {
        return Err(format!("Failed to change ownership with exit status: {exit_status}").into());
    }

    Ok(())
}

fn build_target_path(config: &Config) -> String {
    let mut target_path = config.target_folder.clone();
    if !config.target.is_empty() {
        // if target is provided
        target_path.push_str(&config.target.clone());
        target_path.push('/');
        if config.debug {
            target_path.push_str("debug/");
        } else {
            // Use the configured profile name for the path
            target_path.push_str(&format!("{}/", config.profile));
        }
    }
    target_path
}
