// Load Modules
pub mod constants;
pub mod pw_horse;
// Load functions
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    config: Option<String>,
    #[arg(long)]
    corpus: Option<String>,
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let corpus = genpass::load_words(args.corpus);
    println!("{}", pw_horse::Password::new(corpus));
}
