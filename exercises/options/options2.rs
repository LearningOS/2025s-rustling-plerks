// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        /* if let比较的是enum实例的类型，确认为Some后通过解构拿到了Some中的值到word中。
        类似于if optional_target.type_id == Option::Some::type_id { word = optional_target(Some).data }
        */
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        while let Some(Some(integer)) = optional_integers.pop() { // vector的元素类型记为T，pop()的返回结果就是Option<T>(已经包了一层)，而这里vector的元素类型为Option，所以=左边要解构两层
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
