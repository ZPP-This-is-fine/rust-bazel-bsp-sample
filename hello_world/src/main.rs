use clap::Parser;
use clap::builder::TypedValueParser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Long desc")]
struct TestClapArgs {
    #[arg(
    long,
    default_value_t = 42,
    value_parser = clap::builder::PossibleValuesParser::new(["42", "21", "37"])
    .map(|s| s.parse::<i32>().unwrap()),
    )]
    my_funny_number: i32,
}

fn main() {
    let args = TestClapArgs::parse();
    println!("Your my_funny_number args: {}", args.my_funny_number);
}
