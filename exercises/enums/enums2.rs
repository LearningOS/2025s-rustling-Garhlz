// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move{x:i32,y:i32},
    Echo(String),
    ChangeColor(u8,u8,u8,),
    Quit, // 实际是单元类型的结构体
}

impl Message {
    fn call(&self) {
        println!("{:?}", self); // 直接调用debug方法使用各个枚举即可
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
