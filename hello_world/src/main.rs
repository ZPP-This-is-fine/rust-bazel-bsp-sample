use hello_macro::AnswerFn;

use clap::builder::TypedValueParser;
use clap::Parser;

#[derive(AnswerFn)]
#[allow(dead_code)]
struct Struct;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Long desc")]
struct TestClapArgs {
    #[arg(
    long,
    default_value_t = 42,
    value_parser = clap::builder::PossibleValuesParser::new(["42", "21", "37"])
    .map(|s| s.parse::<i32>().unwrap()),
    )]
    index: i32,
}

fn main() {
    let args = TestClapArgs::parse();
    println!("Your index args: {}", args.index);

    hello_lib::Greeter::x();
    let hello = hello_lib::Greeter::new("Hello");
    hello.greet("world");
    println!("{}", answer());
    assert_eq!(42, answer());
}
