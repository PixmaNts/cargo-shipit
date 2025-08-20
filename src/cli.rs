pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Cargo subcommand for building and deploying Rust binaries to Linux targets via SSH. Ship it!", long_about = None)]
pub struct CliArgs {
    #[arg(
        short,
        long,
        help = "The path to the configuration file",
        default_value = "shipit.json"
    )]
    pub config: String,
    #[arg(
        short = 'b',
        long,
        help = "The binary files to upload to the Linux target (auto-detected from Cargo.toml if not specified)"
    )]
    pub binaries: Vec<String>,
    #[arg(short = 'd', long, help = "Enable debug mode", default_value = "false")]
    pub debug: bool,
    #[arg(
        short = 'r',
        long,
        help = "The remote directory path on the Linux target"
    )]
    pub remote_folder: Option<String>,
    #[arg(short = 'H', long, help = "Hostname or IP address of the Linux target")]
    pub host: Option<String>,
    #[arg(short = 'U', long, help = "SSH username for the Linux target")]
    pub username: Option<String>,
    #[arg(short = 'P', long, help = "SSH password for the Linux target")]
    pub password: Option<String>,
    #[arg(
        short = 'k',
        long,
        help = "Path to SSH private key file for authentication"
    )]
    pub key: Option<String>,
    #[arg(
        short = 'p',
        long,
        default_value = "22",
        help = "SSH port of the Linux target"
    )]
    pub port: Option<u16>,
    #[arg(
        short = 't',
        long,
        help = "Rust target triple for cross-compilation (e.g., 'armv7-unknown-linux-gnueabihf', 'aarch64-unknown-linux-gnu')"
    )]
    pub target: Option<String>,
    #[arg(short = 'T', long, help = "Local cargo target directory path")]
    pub target_folder: Option<String>,

    #[arg(
        short,
        long,
        help = "Initialize a new configuration file at the specified path"
    )]
    pub init: Option<String>,

    #[arg(
        short = 'B',
        long,
        default_value = "false",
        help = "Build the project before uploading"
    )]
    pub build: bool,

    #[arg(
        long,
        help = "Build profile to use (debug, release, or custom profile name)",
        default_value = "release"
    )]
    pub profile: String,
}
