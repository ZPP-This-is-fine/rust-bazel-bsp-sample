extern crate hello_lib;

fn main() {
    hello_lib::Greeter::x();
    let hello = hello_lib::Greeter::new("Hello");
    hello.greet("world");
}
