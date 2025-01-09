use crate::types::Message;

pub async fn handle(message: &Message) {
    println!("listen spelonk");
    match message {
        Message::Discover() => println!("Discover"),
        Message::Update(info) => println!("Update {:?}", info),
    }
}
