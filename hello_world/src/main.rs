
use hello_macro::AnswerFn;

#[derive(AnswerFn)]
struct VeryCoolName;

fn main() {
    hello_lib::Greeter::x();
    let hello = hello_lib::Greeter::new("Hello");
    hello.greet("world");
    println!("The answer is: {}", answer());
    assert_eq!(42, answer());
}
