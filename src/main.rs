use clap::Parser;
use colored::Colorize;
use sha256::try_digest;

/// Compare a given file and sha256sum
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    path: String,
    /// The sha256sum to compare
    pattern: String,
}

fn main() {
    let args = Cli::parse();

    println!("{}", "Comparing File and sha256\n".bold().underline());
    println!("File to comapre: {}", args.path.to_string().yellow());
    println!("Hash to compare: {}", args.pattern.to_string().yellow());

    //sha256 digest file
    let input = args.path;
    let val = try_digest(input).unwrap();

    if val != args.pattern {
        println!("{}", "\n!!! Hashes do not match !!!".red());
    } else {
        println!("{}", "\nHashes match!".green());
    }
}
