// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    // .chars()返回的是迭代器。rust的每个char 4个字节，直接存储Unicode的值(Unicode代码点)。不过&str和String是按UTF-8存的。
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            let mut s = String::new();
            s.push(first.to_ascii_uppercase());
            s.push_str(c.as_str());
            s
        }
    }
    // 这里match内部块的最后一个表达式作为了match这个块的返回结果，但是match本身又是函数的最后一个表达式，因此直接作为了整个函数的返回结果
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut ans = vec![];
    for s in words {
        ans.push(capitalize_first(s));
    }
    ans
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut ans = String::new();
    for s in words {
        let x = capitalize_first(s);
        ans.push_str(x.as_str());
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
