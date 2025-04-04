// enums3.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a
// hint.

enum Message {
    ChangeColor(u8, u8, u8),
    Echo(String),
    Move(Point),
    Quit
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
    message: String
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&mut self, s: String) { self.message = s }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message
        // variants
        // Remember: When passing a tuple as a function argument, you'll need
        // extra parentheses: fn function((t, u, p, l, e))

        /* Message::ChangeColor, Message::Quit这几个确实是enum constructor，但是这里在match里是在匹配并解构，
        Message::ChangeColor(r, g, b)并不是在调用constructor，而是根据enum隐藏的type_id(不一定叫这名，只是表达enum要有个隐藏字段区分enum现在具体是什么类型)，
        去匹配enum实例的类型，并解构出变量。
        */
        match message {
            Message::ChangeColor(r, g, b) => {
                self.change_color((r, g, b));
            },
            Message::Quit => {
                self.quit();
            },
            Message::Echo(e) => {
                self.echo(e);
            },
            Message::Move(m) => {
                self.move_position(m);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            quit: false,
            position: Point { x: 0, y: 0 },
            color: (0, 0, 0),
            message: "hello world".to_string(),
        };
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Echo(String::from("hello world")));
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Quit);

        assert_eq!(state.color, (255, 0, 255));
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.quit, true);
        assert_eq!(state.message, "hello world");
    }
}
