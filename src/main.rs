use resplit::Cli;
use clap::Parser;


fn main() {
    let cli = Cli::parse();
    let buffer = resplit::read_stdin();

    let result = resplit::split(buffer, &cli);
    println!("{}", result);
}
