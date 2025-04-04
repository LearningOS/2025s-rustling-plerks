// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.

pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

/* rust的泛型，必须要用trait指明有哪些函数，否则编译器会直接认定software没有licensing_info()这个函数。
而C++的模板默认有任何函数，当发生实际使用时，才去检查对应的模板实例是否真的有相应的函数。

rust需要用trait表明有哪些函数，这一点上像Java，但是肯定不是像Java那样不同泛型实例共用一份实现，因为不像Java泛型只能装对象且
对象全是size相等的指针。对不同类型必须有不同的实现才行。

rust不把泛型设计成c++的模板那样，虽然要声明trait类型麻烦，但是泛型的报错会更明确。
对于rust，只要调用时符合泛型入口处定义的trait要求，即可知道一定是合法的；
而对于c++，只有到模板内部实际调用的地方才能发现实际类型没有相应的函数。
当编译好泛型后，如果实际调用不合法，rust只需根据泛型入口处的类型要求不满足即可报错，而C++则需要进入模版内部，在内部的行处报错，因此报错信息一大堆。

两种实现方式的特点是，rust在编译泛型时即可知道泛型实现是否合法，而c++要到编译实际使用了模板的代码时才能知道。
*/

fn compare_license_types<T: Licensed, E: Licensed>(software: T, software_two: E) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware {};
        let other_software = OtherSoftware {};

        assert!(compare_license_types(other_software, some_software));
    }
}
