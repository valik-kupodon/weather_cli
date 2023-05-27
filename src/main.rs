use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the weather provider
    #[arg(short, long)]
    configure: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let agrs = Args::parse();
    for _ in 0..agrs.count {
        println!("Hello, {}!", agrs.configure);
    }
}
