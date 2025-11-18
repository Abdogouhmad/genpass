use std::path::PathBuf;

use crate::file::GenFile;
use crate::generating::CredentialGenerator;
use anyhow::Result;
use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser, Subcommand,
};

/// Custom style configuration for colored CLI help
#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    styles = Self::cli_style()
)]
pub struct GenCli {
    #[arg(short, long, help = "Generate password")]
    pub pass: bool,

    #[arg(short, long, help = "Generate username")]
    pub user: bool,
    #[arg(
    short = 'g',
    long = "generate",
    help = "Generate credentials and optionally add a note",
    value_name = "NOTE",
    num_args = 0..=1
)]
    pub generate: Option<Option<String>>,

    #[command(subcommand)]
    pub command: Option<GenCommands>,
}

#[derive(Subcommand, Debug)]
pub enum GenCommands {
    /// Encrypt a file or text
    Encrypt,

    /// Decrypt a file or text
    Decrypt,
}

impl GenCli {
    /// Executes CLI logic
    pub fn run(&self) -> Result<()> {
        let generator = CredentialGenerator::new();
        let password = generator.gen_password()?;
        let username = generator.gen_user()?;
        let custom_path = PathBuf::from("./");

        if self.pass {
            println!("🔐 Generated password: {password}");
        }

        if self.user {
            println!("👤 Generated username: {username}");
        }

        // generate and store in a file
        if let Some(note_option) = &self.generate {
            let note = note_option
                .clone()
                .unwrap_or_else(|| "No note provided".to_string());

            let custom_file = GenFile {
                filename: "password.md".to_string(),
                file_path: custom_path,
                generated_content: format!(
                    "# {}\nUsername: {}\nPassword: {}\n",
                    note, username, password
                ),
            };

            custom_file.create_file()?;
            println!(
                "✅ Successfully saved to {:?}/{}",
                custom_file.file_path, custom_file.filename
            );
        }

        // Encrypt and Dencrypt
        if let Some(cmd) = &self.command {
            match cmd {
                GenCommands::Encrypt => println!("🧩 Encryption selected"),
                GenCommands::Decrypt => println!("🔓 Decryption selected"),
            }
        }

        // if !self.pass && !self.user && self.generate && self.command.is_none() {
        //     println!("⚠️  No option provided. Use --help for available commands.");
        // }
        //
        Ok(())
    }

    /// CLI style
    pub fn cli_style() -> Styles {
        Styles::styled()
            .usage(AnsiColor::BrightRed.on_default())
            .header(AnsiColor::BrightMagenta.on_default())
            .invalid(AnsiColor::BrightRed.on_default())
            .error(AnsiColor::BrightRed.on_default())
            .valid(AnsiColor::BrightBlue.on_default())
            .placeholder(AnsiColor::BrightYellow.on_default())
            .literal(AnsiColor::BrightCyan.on_default())
    }
}
