// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.

/* 对于引用(借用)，rust需要知道生命周期，以防止发生悬空引用，这里longest的返回值是x或者y，
并不确定返回值的生命周期到底是谁的，所以要手动写清楚生命周期。（生命周期的记号用泛型的<>声明）
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);

    let s1 = "111";
    {
        let s2 = "222";
        let res = longest(s1, s2); // 这里s1和s2的生命周期不同，编译器会把'a求为s1和s2生命周期的交集
    }
}
