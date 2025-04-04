// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.

trait AppendBar {
    fn append_bar(self) -> Self;
}

/*
如果方法需要消费原值，用 self
如果方法需要修改原值但保留所有权，用 &mut self
如果方法只需读取，用 &self

这里trait的声明和实现，self有些区别，self和mut self是匹配的，因为转移了所有权到当前方法中，是否mutable可由当前方法决定。
&self和&mut self不匹配，因为涉及外部变量，在是否能修改外部变量上需要保持一致。
*/

impl AppendBar for String {
    fn append_bar(mut self) -> Self {
        self.push_str("Bar");
        self
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar")); // 从这里可以看出，append_bar要返回值，而不是把&mut self传进来改，然后返回void
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
