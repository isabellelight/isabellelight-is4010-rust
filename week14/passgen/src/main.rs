use clap::{Parser, Subcommand};
use passgen::{generate_passphrase, generate_pin, generate_random, validate_strength};

#[derive(Parser)]
#[command(name = "passgen")]
#[command(about = "A secure password generator and validator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a random password
    Random {
        /// Password length
        #[arg(short, long, default_value_t = 16)]
        length: usize,
        /// Include symbols
        #[arg(short, long)]
        symbols: bool,
    },
    /// Generate a passphrase
    Passphrase {
        /// Number of words
        #[arg(short, long, default_value_t = 4)]
        words: usize,
        /// Word separator
        #[arg(short, long, default_value = "-")]
        separator: char,
    },
    /// Generate a numeric PIN
    Pin {
        /// PIN length
        #[arg(short, long, default_value_t = 4)]
        length: usize,
    },
    /// Validate password strength
    Validate {
        /// Password to validate
        password: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Random { length, symbols } => {
            let password = generate_random(length, symbols);
            println!("Generated password: {}", password);
        }
        Commands::Passphrase { words, separator } => {
            let passphrase = generate_passphrase(words, separator);
            println!("Generated passphrase: {}", passphrase);
        }
        Commands::Pin { length } => {
            let pin = generate_pin(length);
            println!("Generated PIN: {}", pin);
        }
        Commands::Validate { password } => {
            let strength = validate_strength(&password);
            println!("Password strength: {:?}", strength);
        }
    }
}
