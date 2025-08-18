use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(conflicts_with = "login")]
    command: String,

    #[arg(allow_hyphen_values = true, conflicts_with = "login")]
    args: Vec<String>,

    #[arg(short, long)]
    login: bool,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.command);
    println!("{:?}", args.args);
    print!("{}", args.login);
    return;
}
