use crate::types::Message;

pub async fn handle(message: &Message) {
    println!("listen the playground");
    match message {
        Message::Discover() => println!("Discover"),
        Message::Update(info) => println!("Update {:?}", info),
    }
}
