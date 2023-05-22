use clap::Parser;

#[derive(Parser, Debug)]
struct Args {

    /// Input files to scan
    file: Vec<String>
}

fn main() {
    let _args = Args::parse();
}
