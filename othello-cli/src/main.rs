/// Entry point for the CLI binary.
fn main() {
    if let Err(err) = othello_cli::run() {
        eprintln!("Application error: {}", err);
        std::process::exit(1);
    }
}