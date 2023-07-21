mod greeting;

use greeting::greet;

#[tokio::main]
async fn main() {
    let greeting = greet("tokio world");
    println!("{}", greeting);
}
