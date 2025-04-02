// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue".to_string() // rust String可以自动转化为&str，但是&str不能自动转化为String(应该是因为&str转String涉及到动态内存分配，所以不隐式转化)
}
