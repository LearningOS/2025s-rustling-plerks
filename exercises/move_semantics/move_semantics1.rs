// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let vec0 = Vec::new();

    // 这里fill_vec()的vec0是传值而非传引用，rust: 
    // 1. 对象传值(=)时使用类似cpp的移动语义，就算是左值也无需std::move，直接就是触发移动。
    // 2. vec0不仅仅是像cpp的移动语义那样对象被搬空，而是这个变量vec0直接被rust编译器认为销毁(所有权转移)，不再可用。
    // 如果是基本数据类型(例如i32)，传值就是发生复制而非移动，不用考虑所有权转移然后变量失效的问题。
    // 使用&创建引用，称为所有权借用，例如`let v = &vec0;`，&要打在被bind的target对象名前面，而非reference variable处。且引用方式传参
    // 时也要写上& (例如func(&vec0))，与c++不同，这样从调用处就能一眼看出传的是值还是引用。

    // rust的`let mut vec = Vec::new();` 类似于cpp的`vector<int> vec;`。不过rust的泛型能延迟推导到第一次使用时，所以没有指定元素类型。
    // 但是只写`let mut vec0 = Vec::new();`，后面一次没有`vec0.push(1);`之类的，不能通过编译。
    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
