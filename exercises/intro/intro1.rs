// intro1.rs
//
// About this `I AM NOT DONE` thing:
// We sometimes encourage you to keep trying things on a given exercise, even
// after you already figured it out. If you got everything working and feel
// ready for the next exercise, remove the `I AM NOT DONE` comment below.
//
// If you're running this using `rustlings watch`: The exercise file will be
// reloaded when you change one of the lines below! Try adding a `println!`
// line, or try changing what it outputs in your terminal. Try removing a
// semicolon and see what happens!
//
// Execute `rustlings hint intro1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    println!("Hello and"); // rust的println是个宏函数，调用时后面要加个!
    println!(r#"       welcome to...                      "#); // r#""# 中间包围的为原始字符串字面量，特殊字符(如\)不用转义
    println!(r#"                 _   _ _                  "#);
    println!(r#"  _ __ _   _ ___| |_| (_)_ __   __ _ ___  "#);
    println!(r#" | '__| | | / __| __| | | '_ \ / _` / __| "#);
    println!(r#" | |  | |_| \__ \ |_| | | | | | (_| \__ \ "#);
    println!(r#" |_|   \__,_|___/\__|_|_|_| |_|\__, |___/ "#);
    println!(r#"                               |___/      "#);
    println!();
    println!("This exercise compiles successfully. The remaining exercises contain a compiler");
    println!("or logic error. The central concept behind Rustlings is to fix these errors and");
    println!("solve the exercises. Good luck!");
    println!();
    println!("The source for this exercise is in `exercises/intro/intro1.rs`. Have a look!");
    println!(
        "Going forward, the source of the exercises will always be in the success/failure output."
    );
    println!();
    println!(
        "If you want to use rust-analyzer, Rust's LSP implementation, make sure your editor is set"
    );
    println!("up, and then run `rustlings lsp` before continuing.")
}
