// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

#[derive(Debug)]
enum Message {
    Move {x: i32, y: i32}, // Move是个struct
    Echo(String), // Echo是个元组，包含一个String
    ChangeColor(u8, u8, u8), // Echo是个元组，包含3个u8
    Quit // Quit没有实体数据
}

/* rust的enum更像union，里面应该是有个隐藏的值，区分当前enum实例是哪种类型，所以就算内容部分相等了，类型不同还是不同。
然后Move, Echo, ChangeColor, Quit这4个全是enum constructor，根据参数把当前enum实例初始化。
*/

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    // println!("{:#?}", Message::Move { x: 10, y: 30 });
    // println!("{:#?}", Message::Quit);

    for message in &messages {
        message.call();
    }
}
