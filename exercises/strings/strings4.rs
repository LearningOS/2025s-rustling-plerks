// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned()); // ToOwned trait目标：引用 -> 值类型的拷贝，impl时定义的to_owned()函数就是从&str返回String
    string_slice("nice weather".into()); // into()会自动转化类型，这里会自动查找调用具体的Into trait实现去把&str给转化了，类似C++定义拷贝构造函数后的自动隐式转化。不过由于&str不能自动转String，这里只能调string_slice()
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
