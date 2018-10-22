

fn hello()
{
    println!("hello function");
}

mod mod_hello;

fn main() {
    hello();
    mod_hello::hello();
    mod_hello::mod_hello2::hello();
    println!("Hello, world!");
}
